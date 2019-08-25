use http::header::HeaderValue;
use http::status::StatusCode;
use tide::{Context, App, response, EndpointResult};
use tide::middleware::{CorsMiddleware, CorsOrigin};
use stremio_core::types::addons::{ResourceRef, ResourceResponse, Manifest, ManifestResource};
use semver::Version;
use std::str::FromStr;

// @TODO another return type
async fn handle_path(ctx: Context<Manifest>) -> EndpointResult {
    let path = ctx.uri().path();
    
    if path == "/manifest.json" {
        return Ok(response::json(ctx.state()));
    }

    let resource = match ResourceRef::from_str(&path) {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::NOT_FOUND.into())
    };

    let msg = ResourceResponse::Streams { streams: vec![] };
    Ok(response::json(msg))
}

fn main() {
    let mut app = App::with_state(Manifest {
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
    });
    // Requires always passing Origin when enabled
    app.middleware(
        CorsMiddleware::new()
            .allow_origin(CorsOrigin::from("*"))
            .allow_methods(HeaderValue::from_static("GET")),
    );
    app.at("/*").get(handle_path);
    app.run("127.0.0.1:8000").unwrap();
}
