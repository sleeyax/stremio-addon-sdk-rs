use stremio_core::state_types::EnvFuture;
use stremio_core::types::addons::{ResourceRef, ResourceResponse};
use futures::{Future};
use std::str::FromStr;
use stremio_core::addon_transport::AddonInterface;
use super::router::{WithHandler, AddonBase};
use super::router::AddonRouter;

async fn handle_manifest(req: tide::Request<Vec<WithHandler<AddonBase>>>) -> tide::Response {
    tide::Response::new(200).body_json(req.state()[0].get_manifest()).unwrap()
}

async fn handle_path(req: tide::Request<Vec<WithHandler<AddonBase>>>) -> tide::Response {
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

pub async fn serve_http(handlers: Vec<WithHandler<AddonBase>>) {
    let mut app = tide::with_state(handlers);
    app.at("/manifest.json").get(handle_manifest);
    app.at("/*").get(handle_path);
    app.listen("127.0.0.1:8000").await.expect("Failed to start server");
}
