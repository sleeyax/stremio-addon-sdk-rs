use super::landing_template::landing_template;
use stremio_core::types::addons::Manifest;
use hyper::{Response, Request, Body, StatusCode, header, Method};
use serde_json;
use now_lambda::IntoResponse;
use super::server::ServerOptions;
use super::builder::BuilderWithHandlers;
use super::builder::AddonRouter;
use futures::stream::Stream;
use futures::future::Future;

pub type Result<T> = std::result::Result<T, RouterError>;

#[derive(Debug)]
pub enum RouterError {
    HttpError(hyper::http::Error),
    SerdeError(serde_json::Error),
}

pub struct RouterResponse {
    response: Response<Body>
}
// implement now.sh lambda response trait
impl IntoResponse for RouterResponse {
    // convert Hyper Response to Now.sh Response
    fn into_response(self) -> Response<now_lambda::Body> {
        let (parts, body) = self.response.into_parts();

        // get original response body as bytes array
        let bytes = body
            .concat2()
            .wait()
            // at least log error
            .map_err(|error| eprintln!("into_response error: {}", error))
            .unwrap()
            .into_bytes();
        let mut bytes_array: Vec<u8> = vec![];
        bytes_array.extend_from_slice(&*bytes);
       
        Response::from_parts(parts, now_lambda::Body::from(bytes_array))
    }
}
// read RouterResponse from Hyper Response
impl From<Response<Body>> for RouterResponse {
    fn from(response: Response<Body>) -> RouterResponse {
        Self {response}
    }
}
impl RouterResponse {
    pub fn response(self) -> Response<Body> {
        self.response
    }

    pub fn response_serverless(self) -> Response<now_lambda::Body> {
        self.into_response()
    }
}

pub struct Router {
    build: BuilderWithHandlers,
    options: ServerOptions
}
impl Router {
    pub fn new(build: BuilderWithHandlers, options: ServerOptions) -> Self {
        Self {build, options}
    }

    fn get_manifest(&self) -> &Manifest {
        self.build.handlers[0].get_manifest()
    }

    fn json_response(&self, json: String) -> Result<Response<Body>> {
        Response::builder()
            .status(StatusCode::OK)
            .header("access-control-allow-origin", "*") // CORS
            .header("Cache-Control", format!("max-age={}, public", self.options.cache_max_age)) // cache
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .map_err(RouterError::HttpError)
    }

    fn html_response(&self, html: String) -> Result<Response<Body>> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(html))
            .map_err(RouterError::HttpError)
    }

    fn not_found(&self) -> Result<Response<Body>> {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .map_err(RouterError::HttpError)
    }

    fn method_not_allowed(&self) -> Result<Response<Body>> {
        Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("Method not allowed"))
            .map_err(RouterError::HttpError)
    }

    pub fn handle_manifest(&self) -> Result<Response<Body>> {
        let json = serde_json::to_string(self.get_manifest())
            .map_err(RouterError::SerdeError)?;
        self.json_response(json)
    }

    pub fn handle_resource(&self, path: &str) -> Result<Response<Body>> {
        let res = match self.build.handle(path) {
			Some(res) => res,
			None => return self.not_found()
        };

        let json = serde_json::to_string(&res)
            .map_err(RouterError::SerdeError)?;
        self.json_response(json)
    }

    pub fn handle_landing(&self, template: String) -> Result<Response<Body>> {
        self.html_response(template)
    }

    pub fn handle_default_landing(&self) -> Result<Response<Body>> {
        self.handle_landing(landing_template(self.get_manifest()))
    }

    pub fn route<T>(&self, request: Request<T>) -> Result<RouterResponse> {
        if request.method() != Method::GET {
            return Ok(RouterResponse::from(self.method_not_allowed()?));
        }

        let path = request.uri().path();
        
        Ok(RouterResponse::from(
            match path {
                "/manifest.json" => self.handle_manifest()?,
                "/" => self.handle_default_landing()?,
                _ => self.handle_resource(path)?,
            }
        ))
    }
}
