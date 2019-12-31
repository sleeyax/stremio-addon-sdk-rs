use stremio_core::state_types::EnvFuture;
use http::request::Request;
use http::header::HeaderValue;
use http::status::StatusCode;
use tide::{Context, App, response, EndpointResult};
use tide::middleware::{CorsMiddleware, CorsOrigin};
use stremio_core::types::addons::{ResourceRef, ResourceResponse, Manifest, ManifestResource};
use semver::Version;
use stremio_core::addon_transport::AddonInterface;
use std::str::FromStr;
use futures::{future, Future};
use super::router::{WithHandler, AddonBase};
use super::router::AddonRouter;

async fn handle_manifest(ctx: Context<WithHandler<AddonBase>>) -> EndpointResult {
    Ok(response::json(ctx.state().get_manifest()))
}

async fn handle_path(ctx: Context<WithHandler<AddonBase>>) -> EndpointResult {
    let path = ctx.uri().path();
    
    let resource = match ResourceRef::from_str(&path) {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::NOT_FOUND.into())
    };
    dbg!(&resource);
    
    // let future = ctx.state().get(&resource);

    let msg = ResourceResponse::Streams { streams: vec![] };
    Ok(response::json(msg))
}

pub fn serve_http(handler: WithHandler<AddonBase>) {
    let mut app = App::with_state(handler);
    // Requires always passing Origin when enabled
    app.middleware(
        CorsMiddleware::new()
            .allow_origin(CorsOrigin::from("*"))
            .allow_methods(HeaderValue::from_static("GET")),
    );
    app.at("/manifest.json").get(handle_manifest);
    app.at("/*").get(handle_path);
    app.run("127.0.0.1:8000").unwrap();
}
