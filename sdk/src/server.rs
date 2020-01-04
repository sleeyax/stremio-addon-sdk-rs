use stremio_core::state_types::EnvFuture;
use stremio_core::types::addons::{ResourceRef, ResourceResponse};
use futures::{Future};
use std::str::FromStr;
use stremio_core::addon_transport::AddonInterface;
use tide::middleware::{Cors, Origin};
use http::header::HeaderValue;
use super::router::{WithHandler, AddonBase};
use super::router::AddonRouter;
use super::landing_template::landing_template;
use super::cache_middleware::Cache;

type Handlers = Vec<WithHandler<AddonBase>>;

async fn handle_manifest(req: tide::Request<Handlers>) -> tide::Response {
    tide::Response::new(200).body_json(req.state()[0].get_manifest()).unwrap()
}

async fn handle_landing(req: tide::Request<Handlers>) -> tide::Response {
    tide::Response::new(200)
        .body_string(
            landing_template(req.state()[0].get_manifest())
        )
        .set_header("Content-Type", "text/html")
}

async fn handle_path(req: tide::Request<Handlers>) -> tide::Response {
    // get requested resource
    let path = req.uri().path();
    let resource = match ResourceRef::from_str(&path) {
        Ok(r) => r,
        Err(_) => return tide::Response::new(404)
    };
    dbg!(&resource);

    // find correct handler for this resource
    let handlers: &Vec<WithHandler<AddonBase>> = &req.state();
    let handler_option = handlers.iter().find(|&item| path.starts_with(&item.match_prefix));
    let handler = match handler_option {
        Some(x) => x,
        _ => return tide::Response::new(404)
    };
    
    // execute the handler
    let env_future: EnvFuture<ResourceResponse> = handler.get(&resource);
    let resource_response = match env_future.wait() {
        Ok(r) => r,
        Err(_) => return tide::Response::new(500)
    };
    dbg!(&resource_response);

    // let msg = ResourceResponse::Streams { streams: vec![] };
    tide::Response::new(200).body_json(&resource_response).unwrap()
}

pub struct ServerOptions {
    pub port: i16,
    pub cache_max_age: Option<i32>
}
impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            // cache 3 days
            cache_max_age: Some(24 * 3600 * 3),
            port: 7070
        }
    }
}

pub async fn serve_http(handlers: Handlers, options: ServerOptions) {
    let mut app = tide::with_state(handlers);
    app.middleware(
        Cors::new()
            .allow_methods(HeaderValue::from_static("GET, POST, OPTIONS"))
            .allow_origin(Origin::from("*"))
            .allow_credentials(false)
    );
    if let Some(cache) = options.cache_max_age {
        app.middleware(
            Cache::new(cache)
        );
    }
    app.at("/manifest.json").get(handle_manifest);
    app.at("/").get(handle_landing);
    app.at("/*").get(handle_path);
    app.listen(format!("127.0.0.1:{}", options.port)).await.expect("Failed to start server");
}
