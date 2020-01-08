use stremio_core::types::addons::{ResourceRef, ManifestResource, ParseResourceErr, ResourceResponse, Manifest};
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct WithHandler<T: AddonRouter> {
    base: T,
    pub match_prefix: String,
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

// Builder: constructs a new builder that implements WithHandler
pub struct Builder;
impl Builder {
    pub fn new(manifest: Manifest) -> BuilderWithHandlers {
        // typestate, two different types for the builder, when we attach handlers
        // so that we cannot build before that
        BuilderWithHandlers {
            handlers: vec![],
            base: AddonBase { manifest }
        }
    }
}

// BuilderWithHandlers: builder with handlers attached
#[derive(Clone)]
pub struct BuilderWithHandlers {
    base: AddonBase,
    pub handlers: Vec<WithHandler<AddonBase>>
}
impl BuilderWithHandlers {
    fn handle_resource(&mut self, resource_name: &str, handler: Handler) -> &mut Self {
        if self.handlers.iter().any(|h| self.prefix_to_name(&h.match_prefix) == resource_name) {
            panic!("handler for resource {} is already defined!", resource_name);
        }
        self.handlers.push(WithHandler {
            base: self.base.clone(),
            match_prefix: format!("/{}/", resource_name),
            handler
        });
        self
    }
    pub fn define_stream_handler(&mut self, handler: Handler) -> &mut Self {
        self.handle_resource("stream", handler)
    }
    pub fn define_meta_handler(&mut self, handler: Handler) -> &mut Self {
        self.handle_resource("meta", handler)
    }
    pub fn define_catalog_handler(&mut self, handler: Handler) -> &mut Self {
        self.handle_resource("catalog", handler)
    }
    pub fn define_subtitles_handler(&mut self, handler: Handler) -> &mut Self {
        self.handle_resource("subtitles", handler)
    }
    pub fn handle(&self, path: &str) -> Option<EnvFuture<ResourceResponse>> {
        // get requested resource
        let resource = match ResourceRef::from_str(path) {
            Ok(r) => r,
            Err(_) => return None
        };
        dbg!(&resource);

        // find correct handler for this resource
        let handler = match self.handlers.iter().find(|&item| path.starts_with(&item.match_prefix)) {
            Some(x) => x,
            _ => return None
        };
        
        // execute the handler
        let env_future: EnvFuture<ResourceResponse> = handler.get(&resource);
        Some(env_future)
    }
    fn prefix_to_name(&self, prefix: &String) -> String {
        prefix.replace("/", "")
    }
    fn validate(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        let manifest = self.base.get_manifest();

        if self.handlers.len() == 0 {
            errors.push("at least one handler must be defined".into());
        }
        
        // get all handlers that are declared in the maifest
        let mut handlers_in_manifest: Vec<String> = Vec::new();
        if manifest.catalogs.len() > 0 {
            handlers_in_manifest.push("catalog".into());
        }
        for resource in &manifest.resources {
            // NOTE: resource.name() should probably be public in stremio-core, making this code unnecessary
            match resource {
                ManifestResource::Short(n) => handlers_in_manifest.push(n.to_string()),
                ManifestResource::Full { name, .. } => handlers_in_manifest.push(name.to_string()),
            }
        }
        
        // check if defined handlers are also specified in the manifest
        for defined_handler in &self.handlers {
            if !handlers_in_manifest.iter().any(|r| r.to_string() == self.prefix_to_name(&defined_handler.match_prefix)) {
                if defined_handler.match_prefix == "/catalog/" {
                    errors.push("manifest.catalogs is empty, catalog handler will never be called".into());
                }
                else {
                    errors.push(format!("manifest.resources does not contain: {}", self.prefix_to_name(&defined_handler.match_prefix)));
                }
            }
        }

        // check if handlers that are specified in the manifest are also defined
        for handler in handlers_in_manifest {
            if !self.handlers.iter().any(|r| handler == self.prefix_to_name(&r.match_prefix)) {
                errors.push(format!("manifest definition requires handler for {}, but it is not provided", handler));
            }
        }

        return errors;
        
    }
    pub fn build(&self) -> Self {
        let errors = self.validate();
        if errors.len() > 0 {
            panic!(format!("\n--failed to build addon interface-- \n{}", errors.join("\n")));
        }
        self.clone()
    }
}

impl<T: AddonRouter> AddonInterface for WithHandler<T> {
    fn manifest(&self) -> EnvFuture<Manifest> {
        Box::new(future::ok(self.get_manifest().to_owned()))
    }
    fn get(&self, req: &ResourceRef) -> EnvFuture<ResourceResponse> {
        Box::new(self.route(&req.to_string()).map_err(Into::into))
    }
}

