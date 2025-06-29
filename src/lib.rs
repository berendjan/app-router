/// A trait for routing messages to handlers asynchronously.
///
/// # Type Parameters
/// - `Handler`: The handler type.
/// - `Message`: The message type to route.
/// - `Response`: The response type returned by the route (defaults to `()`).
pub trait AppRoute<Handler, Message, Response = ()> {
    /// Routes a message to the appropriate handler asynchronously.
    ///
    /// # Arguments
    /// * `message` - The message to route.
    ///
    /// # Returns
    /// An async future resolving to the response.
    fn route(&self, message: &Message) -> impl std::future::Future<Output = Response> + Send;
}

/// A trait for sending messages to a [`AppRoute`](crate::AppRoute) asynchronously.
///
/// # Type Parameters
/// - `Message`: The message type to send.
/// - `AppRouter`: The router type implementing [`AppRoute`](crate::AppRoute).
/// - `Response`: The response type returned by the send operation (defaults to `()`).
pub trait Sender<Message, Router, Response = ()> {
    /// Sends a message to the router asynchronously.
    ///
    /// # Arguments
    /// * `message` - The message to send.
    /// * `router` - The router to send the message to.
    ///
    /// # Returns
    /// An async future resolving to the response.
    fn send(
        message: &Message,
        router: &Router,
    ) -> impl std::future::Future<Output = Response> + Send;
}

/// Macro to generate a app router struct and implementations for routing and sending messages.
///
/// # Syntax
/// ```ignore
/// app_router! {
///     handlers: [ handler1: HandlerType1, handler2: HandlerType2, ... ]
///     routes: [
///         SourceType1, MessageType1: [ handler1, handler2, ... ],
///         SourceType2, MessageType2, ResponseType2: [ handler1, handler2 ],
///         ...
///     ]
/// }
/// ```
///
/// - `handlers`: List of handler fields and their types for the router struct.
/// - `routes`: List of routing rules mapping (source, message, [optional response]) to handlers.
///
/// The response should match the returned type of the last handler in the list.
///
/// This macro generates:
/// - A `AppRouter` struct with the specified handlers.
/// - Implementations of [`Sender`](crate::Sender) for each route source.
/// - Implementations of [`AppRoute`](crate::AppRoute) for the router.
///
/// The
///
/// # Example
/// ```ignore
/// app_router! {
///     handlers: [ foo: FooHandler, bar: BarHandler ]
///     routes: [
///         FooSource, FooMsg: [foo],
///         BarSource, BarMsg, BarResp: [foo,bar]
///     ]
/// }
/// ```
#[macro_export]
macro_rules! app_router {
    (
        $( derive: [ $( $derive:ident ),+ $(,)? ], )?
        handlers: [ $( $handler_ident:ident: $handler_ty:ty ),+ $(,)? ]
        routes: [ $( $source:ty, $message:ty$(, $response:ty)?: [ $( $receiver:ident ),+ ] ),+ $(,)? ]
    ) => {


        $( #[derive( $( $derive ),+ )] )?
        pub struct AppRouter {
            $(
                pub $handler_ident: $handler_ty,
            )+
        }

        mod trait_impls {
            use super::*;
            use app_router::{AppRoute, Sender};

            $(
                impl Sender<$message, AppRouter$(, $response)?> for $source {
                    #[inline]
                    async fn send(message: &$message, router: &AppRouter)$( -> $response)? {
                        AppRoute::<Self, $message$(, $response)?>::route(router, message).await
                    }
                }
            )+

            $(
                impl AppRoute<$source, $message$(, $response)?> for AppRouter {
                    #[inline]
                    async fn route(&self, message: &$message)$( -> $response)? {
                        $(
                            let _out = self.$receiver.handle(message, self).await;
                        )+
                        _out
                    }
                }
            )+
        }
    }
}

/// Example trait for handler implementations.
///
/// See examples for usage without depending on the `AppRouter` type directly.
///
/// # Type Parameters
/// - `Message`: The message type handled.
/// - `Router`: The router type.
/// - `Response`: The response type (defaults to `()`).
pub trait Handle<Message, Router, Response = ()> {
    /// Handles a message asynchronously.
    ///
    /// # Arguments
    /// * `message` - The message to handle.
    /// * `router` - The router instance.
    ///
    /// # Returns
    /// An async future resolving to the response.
    fn handle(
        &self,
        message: &Message,
        router: &Router,
    ) -> impl std::future::Future<Output = Response> + Send;
}
