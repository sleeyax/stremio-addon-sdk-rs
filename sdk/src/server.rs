use hyper::Request;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn_ok};
use hyper::{Body};
use hyper::server::Server;
use hyper::rt::Future;
use super::router::Router;
use super::builder::BuilderWithHandlers;

#[derive(Clone)]
pub struct ServerOptions {
    pub port: i16,
    pub cache_max_age: Option<i32>
}
impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            // cache 3 days
            cache_max_age: Some(24 * 3600 * 3),
            port: 7070
        }
    }
}

pub fn serve_http(build: BuilderWithHandlers, options: ServerOptions) {
    let addr = format!("127.0.0.1:{}", options.port).parse().unwrap();
    
    let service = make_service_fn(move |_: &AddrStream| {
        let router = Router::new(build.clone(), options.clone());
        service_fn_ok(move |req: Request<Body>| {
            router.route(req).response()
        })
    });

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running on http://{}", addr);

    hyper::rt::run(server)
}
