use stremio_core::types::addons::{ResourceRef, ParseResourceErr, ResourceResponse, Manifest};
use stremio_core::addon_transport::AddonInterface;
use stremio_core::state_types::EnvFuture;
use futures::{future, Future};
use std::error::Error;
use std::str::FromStr;

type Handler = fn (req: &ResourceRef) -> EnvFuture<ResourceResponse>;
type RouterFut = Box<dyn Future<Item=ResourceResponse, Error=RouterErr>>;

#[derive(Debug)]
pub enum RouterErr {
    NotFound,
    Handler(Box<dyn Error>),
    Parse(ParseResourceErr)
}
pub trait AddonRouter {
    fn get_manifest(&self) -> &Manifest;
    fn route(&self, path: &str) -> RouterFut;
}
impl Error for RouterErr {}
impl std::fmt::Display for RouterErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RouterError")
    }
}

// Base: just serving the manifest
pub struct AddonBase {
    manifest: Manifest
}
impl AddonRouter for AddonBase {
    fn get_manifest(&self) -> &Manifest {
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
    fn get_manifest(&self) -> &Manifest {
        self.base.get_manifest()
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
// @TODO: typestate, two different types for the builder, when we attach handlers
// so that we cannot build before that
pub enum Builder {
    WithManifest(AddonBase),
    WithHandlers(WithHandler<AddonBase>),
}
impl Builder {
    pub fn new(manifest: Manifest) -> Self {
        Self::WithManifest(AddonBase { manifest })
    }
    pub fn handle_resource(self, resource_name: &str, handler: Handler) -> Self {
        Self::WithHandlers(WithHandler {
            base: match self {
                Self::WithManifest(x) => x,
                Self::WithHandlers(x) => unreachable!("TODO: allow adding more handlers")
            },
            match_prefix: format!("/{}/", resource_name),
            handler
        })
    }
    pub fn build(self) -> WithHandler<AddonBase> {
        // @TODO we can check whether all resources in the manifest are defined
        match self {
            Self::WithManifest(_) => panic!("you must define handlers"),
            Self::WithHandlers(x) => x
        }
    }
}

pub struct BuilderWithHandlers {}

// @TODO
// it just needs an Error trait on RouterErr

impl<T: AddonRouter> AddonInterface for WithHandler<T> {
    fn manifest(&self) -> EnvFuture<Manifest> {
        Box::new(future::ok(self.get_manifest().to_owned()))
    }
    fn get(&self, req: &ResourceRef) -> EnvFuture<ResourceResponse> {
        Box::new(self.route(&req.to_string()).map_err(Into::into))
    }
}

