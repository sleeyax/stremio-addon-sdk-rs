pub mod builder;
pub mod server;
pub mod landing_template;
pub mod router;
pub mod helpers;
pub mod export {
    pub mod serverless {
        pub mod now {
            pub use now_lambda::Request;
            pub use now_lambda::IntoResponse;
            pub use now_lambda::error::NowError;
        }
    }
}
pub mod scaffold;

#[cfg(test)]
mod tests {
    use super::*;
    use stremio_core::types::addons::*;
    use futures::future;

    #[test]
    #[should_panic]
    fn builder_panics_if_no_handlers_attached() {
        builder::Builder::new(scaffold::Scaffold::default_manifest()).build();
    }

    #[test]
    #[should_panic]
    fn builder_panics_if_no_resources_defined_for_handler() {
        builder::Builder::new(scaffold::Scaffold::default_manifest())
            .define_stream_handler(|_| Box::new(future::ok(ResourceResponse::Streams {streams: vec![]})))
            .build();
    }

    #[test]
    #[should_panic]
    fn builder_panics_if_no_handlers_defined_for_resource() {
        let manifest = Manifest {
            resources: vec![ManifestResource::Short("meta".into()), ManifestResource::Short("stream".into())],
            ..scaffold::Scaffold::default_manifest()
        };
        builder::Builder::new(manifest)
            .define_stream_handler(|_| Box::new(future::ok(ResourceResponse::Streams {streams: vec![]})))
            .build();
    }
}