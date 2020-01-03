use stremio_core::state_types::EnvFuture;
use semver::Version;
use stremio_core::types::addons::*;
use stremio_core::types::*;
use stremio_addon_sdk::router::Builder;
use stremio_addon_sdk::server::{serve_http, ServerOptions};
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
    // bbbcatalog
    Box::new(future::ok(ResourceResponse::Metas {metas: vec![
        MetaPreview {
            id: "tt1254207".into(),
            name: "Big buck Bunny".into(),
            poster: Some("https://image.tmdb.org/t/p/w600_and_h900_bestv2/uVEFQvFMMsg4e6yb03xOfVsDz4o.jpg".into()),
            poster_shape: PosterShape::default(),
            description: Some("addon test".into()),
            genres: vec![],
            logo: None,
            release_info: None,
            released: None,
            runtime: None,
            trailer: None,
            type_name: "others".into()
        }
    ]}))
}

#[tokio::main]
async fn main() {
    let manifest = Manifest {
        id: "org.test".into(),
        name: "Rust Example Addon".into(),
        version: Version::new(1, 0, 0),
        resources: vec![
            ManifestResource::Short("catalog".into()),
            ManifestResource::Short("stream".into())
        ],
        types: vec!["movie".into()],
        catalogs: vec![
            ManifestCatalog {
                type_name: "others".into(),
                id: "bbbcatalog".into(),
                name: Some("Rust test".into()),
                extra: ManifestExtra::default()
            }
        ],
        contact_email: None,
        background: None,
        logo: None,
        id_prefixes: Some(vec!["tt".into()]),
        description: Some("Rust addon test".into()),
        addon_catalogs: vec![],
        behavior_hints:  Default::default() // serde_json::map::Map::new()
    };

    // build addon interface
    let interface = Builder::new(manifest)
        .define_catalog_handler(handle_catalog)
        .define_stream_handler(handle_stream)
        .build();

    // HTTP server settings
    let options = ServerOptions {
        cache_max_age: Some(9999),
        port: 1337
    };

    serve_http(interface, options).await;
}
