use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Context<T>(pub Arc<T>);

impl<T> Context<T> {
    pub fn new(value: T) -> Self
    where
        T: Send + Sync,
    {
        Context(Arc::new(value))
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

    pub fn insert<T: 'static + Send + Sync>(&self, value: T) {
        self.dependencies
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), Arc::new(value));
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Arc<T> {
        let map = self.dependencies.read().unwrap();

        let value = map.get(&TypeId::of::<T>()).unwrap_or_else(|| {
            panic!(
                "ContextStore: dependency not found for type {:?}",
                std::any::type_name::<T>()
            )
        });

        value.clone().downcast::<T>().unwrap_or_else(|_| {
            panic!(
                "ContextStore: type mismatch when downcasting {}",
                std::any::type_name::<T>()
            )
        })
    }
}
