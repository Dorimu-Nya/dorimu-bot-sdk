// use crate::{Error, EventContext, EventResponse, Middleware, Next, Result};
// use async_trait::async_trait;
// use std::{collections::HashMap, future::Future, sync::Arc};
//
// #[async_trait]
// pub trait EventHandler: Send + Sync {
//     async fn handle(&self, ctx: EventContext) -> Result<EventResponse>;
// }
//
// pub struct HandlerFn<F>(pub F);
//
// #[async_trait]
// impl<F, Fut> EventHandler for HandlerFn<F>
// where
//     F: Fn(EventContext) -> Fut + Send + Sync + 'static,
//     Fut: Future<Output = Result<EventResponse>> + Send + 'static,
// {
//     async fn handle(&self, ctx: EventContext) -> Result<EventResponse> {
//         (self.0)(ctx).await
//     }
// }
//
// #[derive(Default, Clone)]
// pub struct EventRouter {
//     handlers: HashMap<String, Arc<dyn EventHandler>>,
//     default_handler: Option<Arc<dyn EventHandler>>,
//     middlewares: Vec<Arc<dyn Middleware>>,
// }
//
// impl EventRouter {
//     pub fn new() -> Self {
//         Self::default()
//     }
//
//     pub fn route<H>(mut self, event_name: impl Into<String>, handler: H) -> Self
//     where
//         H: EventHandler + 'static,
//     {
//         self.handlers.insert(event_name.into(), Arc::new(handler));
//         self
//     }
//
//     pub fn route_fn<F, Fut>(self, event_name: impl Into<String>, f: F) -> Self
//     where
//         F: Fn(EventContext) -> Fut + Send + Sync + 'static,
//         Fut: Future<Output = Result<EventResponse>> + Send + 'static,
//     {
//         self.route(event_name, HandlerFn(f))
//     }
//
//     pub fn default_handler<H>(mut self, handler: H) -> Self
//     where
//         H: EventHandler + 'static,
//     {
//         self.default_handler = Some(Arc::new(handler));
//         self
//     }
//
//     pub fn default_handler_fn<F, Fut>(self, f: F) -> Self
//     where
//         F: Fn(EventContext) -> Fut + Send + Sync + 'static,
//         Fut: Future<Output = Result<EventResponse>> + Send + 'static,
//     {
//         self.default_handler(HandlerFn(f))
//     }
//
//     pub fn middleware<M>(mut self, mw: M) -> Self
//     where
//         M: Middleware + 'static,
//     {
//         self.middlewares.push(Arc::new(mw));
//         self
//     }
//
//     pub async fn dispatch(&self, ctx: EventContext) -> Result<EventResponse> {
//         let handler = match ctx.event_name.clone() {
//             Some(name) => self
//                 .handlers
//                 .get(&name)
//                 .cloned()
//                 .or_else(|| self.default_handler.clone()),
//             None => self.default_handler.clone(),
//         }
//         .ok_or(Error::EventNameNotFound)?;
//
//         let mut next = Next::new(move |ctx| {
//             let handler = handler.clone();
//             Box::pin(async move { handler.handle(ctx).await })
//         });
//
//         let middlewares = self.middlewares.clone();
//         for mw in middlewares.into_iter().rev() {
//             let prev = next.clone();
//             next = Next::new(move |ctx| {
//                 let mw = mw.clone();
//                 let prev = prev.clone();
//                 Box::pin(async move { mw.handle(ctx, prev).await })
//             });
//         }
//
//         next.run(ctx).await
//     }
// }
