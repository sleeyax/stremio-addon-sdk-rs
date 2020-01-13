use stremio_addon_sdk::builder::BuilderWithHandlers;
use stremio_core::state_types::EnvFuture;
use stremio_core::types::addons::*;
use stremio_core::types::*;
use stremio_addon_sdk::builder::Builder;
use futures::future;

fn handle_stream(resource: &ResourceRef) -> EnvFuture<ResourceResponse> {
    let mut streams = vec![];
    if resource.type_name.eq("movie") && resource.id.eq("tt1254207") {
        streams.push(Stream {
            title: Some("[RUST TEST] Big buck bunny".into()),
            source: StreamSource::Url {
                url: "http://distribution.bbb3d.renderfarming.net/video/mp4/bbb_sunflower_1080p_30fps_normal.mp4".into()
            },
            behavior_hints: Default::default(),
            thumbnail: None,
            subtitles: vec![],
        });
    }
    dbg!(&streams);
    
    return Box::new(future::ok(ResourceResponse::Streams {streams}));
}

/* fn handle_meta(req: &ResourceRef) -> EnvFuture<ResourceResponse> {
    let res = ResourceResponse::Metas { metas: vec![] };
    return Box::new(future::ok(res));
} */

fn handle_catalog(_resource: &ResourceRef) -> EnvFuture<ResourceResponse> {
    Box::new(future::ok(ResourceResponse::Metas {metas: vec![
        MetaPreview {
            id: "tt1254207".into(),
            name: "Big buck Bunny".into(),
            poster: Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/uVEFQvFMMsg4e6yb03xOfVsDz4o.jpg".into()),
            description: Some("addon test".into()),
            type_name: "others".into(),
            ..Default::default()
        }
    ]}))
}

pub fn build(manifest: Manifest) -> BuilderWithHandlers {
     Builder::new(manifest)
        .define_catalog_handler(handle_catalog)
        .define_stream_handler(handle_stream)
        .build()
}