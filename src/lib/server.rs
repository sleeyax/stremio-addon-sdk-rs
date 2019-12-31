use stremio_core::state_types::EnvFuture;
use stremio_core::types::addons::{ResourceRef, ResourceResponse};
use futures::{Future};
use std::str::FromStr;
use stremio_core::addon_transport::AddonInterface;
use super::router::{WithHandler, AddonBase};
use super::router::AddonRouter;

async fn handle_manifest(req: tide::Request<WithHandler<AddonBase>>) -> tide::Response {
    tide::Response::new(200).body_json(req.state().get_manifest()).unwrap()
}

async fn handle_path(req: tide::Request<WithHandler<AddonBase>>) -> tide::Response {
    let path = req.uri().path();
    let resource = match ResourceRef::from_str(&path) {
        Ok(r) => r,
        Err(_) => return tide::Response::new(404)
    };
    dbg!(&resource);
    
    let env_future: EnvFuture<ResourceResponse> = req.state().get(&resource);
    let resource_response = match env_future.wait() {
        Ok(r) => r,
        Err(_) => return tide::Response::new(500)
    };
    dbg!(&resource_response);

    // let msg = ResourceResponse::Streams { streams: vec![] };
    tide::Response::new(200).body_json(&resource_response).unwrap()
}

pub async fn serve_http(handler: WithHandler<AddonBase>) {
    let mut app = tide::with_state(handler);
    app.at("/manifest.json").get(handle_manifest);
    app.at("/*").get(handle_path);
    app.listen("127.0.0.1:8000").await.expect("Failed to start server");
}
