use app_router::{Handle, Sender, app_router};

struct MyMessage;
struct MySource;
struct Sink;

// Example usage of the macro
app_router! {
    handlers: [ my_source: MySource, sink: Sink ]
    routes: [
        MySource, MyMessage, String: [sink],
    ]
}

impl Handle<MyMessage, AppRouter, String> for Sink {
    async fn handle(&self, _message: &MyMessage, _router: &AppRouter) -> String {
        "Handled by Sink".to_string()
    }
}

#[tokio::test]
async fn test_app_router() {
    let app_router = AppRouter {
        my_source: MySource,
        sink: Sink,
    };
    let out = MySource::send(&MyMessage, &app_router).await;
    assert_eq!(out, "Handled by Sink");
}
