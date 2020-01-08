use crate::router::BuilderWithHandlers;
use futures::{Future};
use tide::middleware::{Cors, Origin};
use http::header::HeaderValue;
use super::router::AddonRouter;
use super::landing_template::landing_template;
use super::cache_middleware::Cache;

async fn handle_manifest(req: tide::Request<BuilderWithHandlers>) -> tide::Response {
    tide::Response::new(200).body_json(req.state().handlers[0].get_manifest()).unwrap()
}

async fn handle_landing(req: tide::Request<BuilderWithHandlers>) -> tide::Response {
    tide::Response::new(200)
        .body_string(
            landing_template(req.state().handlers[0].get_manifest())
        )
        .set_header("Content-Type", "text/html")
}

async fn handle_path(req: tide::Request<BuilderWithHandlers>) -> tide::Response {
    let path = req.uri().path();
    let resource_response_future = match req.state().handle(path) {
        Some(r) => r,
        None => return tide::Response::new(404)
    };
    let resource_response = match resource_response_future.wait() {
        Ok(r) => r,
        Err(_) => return tide::Response::new(500)
    };
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

pub async fn serve_http(builder: BuilderWithHandlers, options: ServerOptions) {
    let mut app = tide::with_state(builder);
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
