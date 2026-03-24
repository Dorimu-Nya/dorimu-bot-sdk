// use crate::{EventContext, EventResponse, Result};
// use async_trait::async_trait;
// use std::{future::Future, pin::Pin, sync::Arc};
//
// pub type BoxFutureResult = Pin<Box<dyn Future<Output = Result<EventResponse>> + Send + 'static>>;
//
// #[derive(Clone)]
// pub struct Next {
//     inner: Arc<dyn Fn(EventContext) -> BoxFutureResult + Send + Sync>,
// }
//
// impl Next {
//     pub fn new<F>(f: F) -> Self
//     where
//         F: Fn(EventContext) -> BoxFutureResult + Send + Sync + 'static,
//     {
//         Self { inner: Arc::new(f) }
//     }
//
//     pub async fn run(self, ctx: EventContext) -> Result<EventResponse> {
//         (self.inner)(ctx).await
//     }
// }
//
// #[async_trait]
// pub trait Middleware: Send + Sync {
//     async fn handle(&self, ctx: EventContext, next: Next) -> Result<EventResponse>;
// }
