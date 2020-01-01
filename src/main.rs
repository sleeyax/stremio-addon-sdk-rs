use crate::lib::router::BuilderWithHandlers;
use stremio_core::state_types::EnvFuture;
use semver::Version;
use stremio_core::types::addons::{Manifest, ManifestResource, ResourceRef, ResourceResponse};
mod lib;
use lib::router::Builder;
use futures::{future};
use lib::server::serve_http;

fn handle_stream(req: &ResourceRef) -> EnvFuture<ResourceResponse> {
    let res = ResourceResponse::Streams { streams: vec![] };
    return Box::new(future::ok(res));
}

fn handle_meta(req: &ResourceRef) -> EnvFuture<ResourceResponse> {
    let res = ResourceResponse::Metas { metas: vec![] };
    return Box::new(future::ok(res));
}

#[tokio::main]
async fn main() {
    let manifest = Manifest {
        id: "org.test".into(),
        name: "test".into(),
        version: Version::new(1, 0, 0),
        resources: vec![ManifestResource::Short("stream".into())],
        types: vec!["movie".into()],
        catalogs: vec![],
        contact_email: None,
        background: None,
        logo: None,
        id_prefixes: None,
        description: None,
        addon_catalogs: vec![],
        behavior_hints:  Default::default() // serde_json::map::Map::new()
    };

    let build = Builder::new(manifest)
        .handle_resource("stream", handle_stream)
        .handle_resource("meta", handle_meta)
        .build();

    serve_http(build).await;
}
