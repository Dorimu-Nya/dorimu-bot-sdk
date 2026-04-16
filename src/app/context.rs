use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

pub struct Context<T: ?Sized>(Arc<T>);

impl<T: ?Sized> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: Any + Send + Sync> Context<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(value))
    }
}

impl<T: ?Sized> Context<T> {
    pub fn from_arc(value: Arc<T>) -> Self {
        Self(value)
    }

    pub fn as_arc(&self) -> Arc<T> {
        Arc::clone(&self.0)
    }

    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Context<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[derive(Clone)]
pub struct ContextStore {
    dependencies: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl ContextStore {
    /// 创建空的依赖容器
    pub fn new() -> Self {
        Self {
            dependencies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert<T: Any + Send + Sync>(&self, value: T) -> Option<Arc<dyn Any + Send + Sync>> {
        self.dependencies
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), Arc::new(value))
    }

    pub fn insert_arc<T: Any + Send + Sync>(&self, value: Arc<T>) -> Option<Arc<dyn Any + Send + Sync>> {
        self.dependencies
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), value)
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Arc<T> {
        let map = self.dependencies.read().unwrap();

        let value = map.get(&TypeId::of::<T>()).unwrap_or_else(|| {
            panic!(
                "ContextStore: dependency not found for type {:?}",
                std::any::type_name::<T>()
            )
        });

        Arc::downcast::<T>(value.clone()).unwrap_or_else(|_| {
            panic!(
                "ContextStore: type mismatch when downcasting {}",
                std::any::type_name::<T>()
            )
        })
    }

    pub fn get_context<T: 'static + Send + Sync>(&self) -> Context<T> {
        Context::from_arc(self.get::<T>())
    }
}
