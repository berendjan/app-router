use app_router::{Handle, Sender, app_router};

struct MySource;
struct Sink;

mod middleware;

app_router! {
    handlers: [ my_source: MySource, middleware: middleware::UserMiddleware, sink: Sink ]
    routes: [
        MySource, middleware::MyMessage, String: [middleware],
        middleware::UserMiddleware, middleware::MyMessage, String: [sink],
    ]
}

impl<Router: Sync> Handle<middleware::MyMessage, Router, String> for Sink {
    async fn handle(&self, _message: &middleware::MyMessage, _router: &Router) -> String {
        println!("Handling MyMessage in Sink");
        "Message returned by Sink".to_string()
    }
}

#[tokio::main]
async fn main() {
    // Example usage of the router and message sending
    let app_router = AppRouter {
        my_source: MySource,
        middleware: middleware::UserMiddleware,
        sink: Sink,
    };

    // Sending a message using the MySource sender
    let out = MySource::send(&middleware::MyMessage, &app_router).await;
    println!("{}", out);
}
