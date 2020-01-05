use stremio_addon_sdk::server::{serve_http, ServerOptions};
mod manifest;
use manifest::get_manifest;
mod handlers;
use handlers::get_handlers;

#[tokio::main]
async fn main() {
    // get the Manifest, which is declared in manifest.rs
    let manifest = get_manifest();

    // get the handlers, declared in handlers.rs
    let interface = get_handlers(manifest);

    // HTTP server settings
    let options = ServerOptions {
        cache_max_age: Some(9999),
        port: 1337
    };

    // run HTTP server asynchronously
    serve_http(interface, options).await;
}
