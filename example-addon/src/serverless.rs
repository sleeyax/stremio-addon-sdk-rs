mod handlers;
use stremio_addon_sdk::server::ServerOptions;
use stremio_addon_sdk::server::serve_serverless;
use stremio_addon_sdk::export::serverless::now::*;
use handlers::build;
mod manifest;
use manifest::get_manifest;

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
	let manifest = get_manifest();
	let build = build(manifest);
	let options = ServerOptions::default();
	serve_serverless(req, build, options)
}
