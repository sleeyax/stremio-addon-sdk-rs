use super::landing_template::landing_template;
use stremio_core::types::addons::Manifest;
use hyper::{Response, Request, Body, StatusCode, header, Method};
use serde_json;
use serde_json::Result;
use super::server::ServerOptions;
use super::builder::BuilderWithHandlers;
use super::builder::AddonRouter;

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

    fn json_response(&self, json: Result<String>) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header("access-control-allow-origin", "*") // CORS
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json.expect("Failed to read json")))
            .unwrap() // internal server error TODO: return proper response in case this happens
    }

    fn html_response(&self, html: String) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(html))
            .unwrap()
    }

    fn not_found(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into())
            .unwrap()
    }

    fn method_not_allowed(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body("Method not allowed".into())
            .unwrap()
    }

    pub fn handle_manifest(&self) -> Response<Body> {
        self.json_response(serde_json::to_string(self.get_manifest()))
    }

    pub fn handle_resource(&self, path: &str) -> Response<Body> {
        let res = match self.build.handle(path) {
			Some(r) => r,
			None => return self.not_found()
        };
        
        self.json_response(serde_json::to_string(&res))
    }

    pub fn handle_landing(&self, template: String) -> Response<Body> {
        self.html_response(template)
    }

    pub fn handle_default_landing(&self) -> Response<Body> {
        self.handle_landing(landing_template(self.get_manifest()))
    }

    pub fn route(&self, request: Request<Body>) -> Response<Body> {
        if request.method() != Method::GET {
            return self.method_not_allowed();
        }

        let path = request.uri().path();
        
        match path {
            "/manifest.json" => self.handle_manifest(),
            "/" => self.handle_default_landing(),
            _ => self.handle_resource(path)
        }
    }
}
