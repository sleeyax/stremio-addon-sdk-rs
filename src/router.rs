use stremio_core::types::addons::{ResourceRef, ParseResourceErr, ResourceResponse, Manifest};
use stremio_core::addon_transport::AddonInterface;
use stremio_core::state_types::EnvFuture;
use futures::future;
use std::error::Error;

enum RouterErr {
    NotFound,
    Handler(Box<dyn Error>),
    Parse(ParseResourceErr)
}
trait AddonRouter {
    fn manifest() -> Manifest;
    fn route(path: &str) -> Result<ResourceResponse, RouterErr>;
}

/*
impl<T: AddonRouter> AddonInterface for T {
    fn manifest(&self) -> EnvFuture<Manifest> {
        Box::new(future::ok(T::manifest()))
    }
    fn get(&self, path: &ResourceRef) -> EnvFuture<ResourceResponse> {
        // @TODO
        unimplemented!()
    }
}
*/


// implement the base that just serves the manifest or NotFound
// then implement Handler
