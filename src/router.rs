use stremio_core::types::addons::{ResourceRef, ParseResourceErr, ResourceResponse, Manifest};
use stremio_core::addon_transport::AddonInterface;
use stremio_core::state_types::EnvFuture;
use futures::{future, Future};
use std::error::Error;
use std::str::FromStr;

type Handler = fn (req: &ResourceRef) -> EnvFuture<ResourceResponse>;
type RouterFut = Box<dyn Future<Item=ResourceResponse, Error=RouterErr>>;

pub enum RouterErr {
    NotFound,
    Handler(Box<dyn Error>),
    Parse(ParseResourceErr)
}
pub trait AddonRouter {
    fn manifest(&self) -> &Manifest;
    fn route(&self, path: &str) -> RouterFut;
}

// Base: just serving the manifest
pub struct AddonBase {
    manifest: Manifest
}
impl AddonRouter for AddonBase {
    fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    fn route(&self, _: &str) -> RouterFut {
        Box::new(future::err(RouterErr::NotFound))
    }
}

// WithHandler: attach a handler
pub struct WithHandler<T: AddonRouter> {
    base: T,
    match_prefix: String,
    handler: Handler,
}
impl<T: AddonRouter> AddonRouter for WithHandler<T> {
    fn manifest(&self) -> &Manifest {
        self.base.manifest()
    }

    fn route(&self, path: &str) -> RouterFut {
        if path.starts_with(&self.match_prefix) {
            let res = match ResourceRef::from_str(&path) {
                Ok(r) => r,
                Err(e) => return Box::new(future::err(RouterErr::Parse(e)))
            };
            return Box::new((self.handler)(&res).map_err(|e| RouterErr::Handler(e)));
        }
        self.base.route(&path)
    }
}

// Builder: build something that implements WithHandler

/*
impl AddonInterface for WithHandler {
    fn manifest(_: Self) -> EnvFuture<Manifest> {
        Box::new(future::ok(T::manifest()))
    }
    fn get(_: Self, path: &ResourceRef) -> EnvFuture<ResourceResponse> {
        // @TODO
        unimplemented!()
    }
}
*/

// implement the base that just serves the manifest or NotFound
// then implement Handler
