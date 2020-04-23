use std::net::Ipv4Addr;
use std::env;
use stremio_addon_sdk::server::{serve_http, ServerOptions};

mod manifest;
use manifest::get_manifest;

mod handlers;
use handlers::build;

#[tokio::main]
async fn main() {
    // get the Manifest, which is declared in manifest.rs
    let manifest = get_manifest();

    // get the handlers, declared in handlers.rs
    let interface = build(manifest);

    let port =
        env::var("PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or(1337);

    // HTTP server settings
    let options = ServerOptions {
        cache_max_age: 9999,
        port,
        ip: Ipv4Addr::new(0,0,0,0).into(),
    };

    // run HTTP server asynchronously
    serve_http(interface, options);
}
