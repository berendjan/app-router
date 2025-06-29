use app_router::{Handle, Sender};

pub struct UserMiddleware;

pub struct MyMessage;

// Note does not depend on `AppRouter` type directly
impl<Router: Sync> Handle<MyMessage, Router, String> for UserMiddleware
where
    Self: Sender<MyMessage, Router, String>,
{
    async fn handle(&self, message: &MyMessage, router: &Router) -> String {
        println!("Handling MyMessage in UserMiddleware");
        Self::send(message, router).await
    }
}
