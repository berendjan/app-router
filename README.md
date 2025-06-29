# app-router

A flexible, macro-powered asynchronous message routing library for Rust.

## Features

- **Declarative routing:** Define message routes and handlers with a single macro.
- **Async-first:** All routing and handling is asynchronous.
- **Extensible:** Bring your own handler and message types.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
app-router = "0.1.0"
```

### Create Handlers, Messages and a Router

```rust
struct MySource;
struct MyHandler;
struct MyMessage;

app_router::app_router! {
    handlers: [ my_handler: MyHandler ]
    routes: [
        MySource, MyMessage: [my_handler]
    ]
}
```

### Define Handlers

```rust

impl <Router: Sync> app_router::Handle<MyMessage, Router> for MyHandler {
    async fn handle(&self, message: &MyMessage, router: &Router) {
        // handle message
    }
}

```

### Send a Message

```rust
let router = AppRouter { my_handler: MyHandler {} };
let msg = MyMessage;
MySource::send(&msg, &router).await;
```

## Documentation

See [src/lib.rs](src/lib.rs) for trait and macro documentation.

##