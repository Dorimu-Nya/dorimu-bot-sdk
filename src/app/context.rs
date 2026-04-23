use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

/// 依赖标识, 在宏处理后会将你的以传入你的依赖
///
/// example:
/// ``` rust
/// use qqbot_sdk_macros::command;
/// use qqbot_sdk::Context;
///
/// struct YourContext;
///
/// #[command("/ping")]
/// fn has_context(context: Context<YourContext>) {
///     // Your biz logic...
/// }
/// ```
pub struct Context<T: ?Sized>(Arc<T>);

/// 为 Context 提供克隆能力以共享同一份依赖。
impl<T: ?Sized> Clone for Context<T> {
    /// 克隆当前 Context 并增加底层 Arc 引用计数。
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

/// 为可在线程间共享的具体类型提供 Context 构造方法。
impl<T: Any + Send + Sync> Context<T> {
    /// 用给定值创建一个新的 Context。
    pub fn new(value: T) -> Self {
        Self(Arc::new(value))
    }
}

/// 为 Context 提供 Arc 互转与访问能力。
impl<T: ?Sized> Context<T> {
    /// 通过已有 Arc 包装生成 Context。
    pub fn from_arc(value: Arc<T>) -> Self {
        Self(value)
    }

    /// 获取内部 Arc 的克隆副本。
    pub fn as_arc(&self) -> Arc<T> {
        Arc::clone(&self.0)
    }

    /// 消耗 Context 并返回内部 Arc。
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

/// 允许像引用一样直接访问 Context 内部对象。
impl<T: ?Sized> Deref for Context<T> {
    type Target = T;

    /// 返回内部目标对象的共享引用。
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[derive(Clone)]
pub struct ContextStore {
    dependencies: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

/// 提供按类型存取依赖的上下文容器。
impl ContextStore {
    /// 创建空的依赖容器。
    pub fn new() -> Self {
        Self {
            dependencies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 插入一个依赖并返回被替换的旧值（若存在）。
    pub fn insert<T: Any + Send + Sync>(&self, value: T) -> Option<Arc<dyn Any + Send + Sync>> {
        self.dependencies
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), Arc::new(value))
    }

    /// 插入一个 Arc 依赖并在覆盖旧值时返回类型名。
    pub fn insert_arc<T: Any + Send + Sync>(&self, value: Arc<T>) -> Option<&str> {
        let o = self
            .dependencies
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), value);
        if let Some(_) = o {
            Some(std::any::type_name::<T>())
        } else {
            None
        }
    }

    /// 按类型获取依赖并在缺失或类型不匹配时 panic。
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

    /// 按类型获取依赖并包装为 Context 返回。
    pub fn get_context<T: 'static + Send + Sync>(&self) -> Context<T> {
        Context::from_arc(self.get::<T>())
    }
}
