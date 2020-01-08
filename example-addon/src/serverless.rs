mod handlers;
use stremio_addon_sdk::landing_template::landing_template;
use stremio_addon_sdk::server::ServerOptions;
use handlers::build;
mod manifest;
use manifest::get_manifest;
use http::{StatusCode, header};
use now_lambda::{error::NowError, IntoResponse, Request, Response};
use serde_json;

// TODO: find a way to simplify this POC. Where does tide use the HTTP package? 
// perhaps we can build one general router and wrap tide and now lamdas around it.

// used for now.sh deployments
fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
	let path = req.uri().path();
	let manifest = get_manifest();
	let build = build(manifest.clone());
	let options = ServerOptions::default();

	let mut response = Response::builder();
	let default_response = response
		.status(StatusCode::OK)
		.header("access-control-allow-origin", "*")
		.header(header::CONTENT_TYPE, "application/json");

	if path == "/manifest.json" {
		return Ok(
			default_response
				.body(serde_json::to_string(&manifest).expect("Failed to parse manifest"))
				.expect("Internal server error")
		);
	}
	else if path == "/" {
		return Ok(
			Response::builder()
				.status(StatusCode::OK)
				.header(header::CONTENT_TYPE, "text/html")
				.body(landing_template(&manifest))
				.expect("Internal server error")
		);
	}
	else {
		let res = match build.handle(path) {
			Some(r) => r,
			None => return Ok(Response::builder().status(StatusCode::NOT_FOUND).body("".into()).unwrap())
		};
		return Ok(
			default_response
				.header("Cache-Control", format!("max-age={}, public", if let Some(cache) = options.cache_max_age {cache} else {0} ))
				.body(serde_json::to_string(&res).expect("Failed to get resource"))
				.expect("Internal server error")
		);
	}
}
