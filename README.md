# app-router

A flexible, macro-powered asynchronous message routing library for Rust.

## Features

- **Declarative routing:** Define message routes and handlers with a single macro.
- **Async-first:** All routing and handling is asynchronous.
- **Extensible:** Bring your own handler and message types.

## Usage

See [example](examples/simple/main.rs) for usage. 
The [middleware](examples/simple/middleware.rs) does not depend on the `AppRouter`.

## Documentation

See [src/lib.rs](src/lib.rs) for trait and macro documentation.

##