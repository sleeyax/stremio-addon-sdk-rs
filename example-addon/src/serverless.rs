use http::{StatusCode};
use now_lambda::{error::NowError, IntoResponse, Request, Response};

// used for now.sh deployments
fn handler(_: Request) -> Result<impl IntoResponse, NowError> {
	let response = Response::builder()
		.status(StatusCode::OK)
		.header("Content-Type", "text/plain")
		.body("user endpoint")
		.expect("Internal Server Error");

	Ok(response)
}
