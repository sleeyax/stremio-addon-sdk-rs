use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use hyper::{Request, Response, StatusCode, Body};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn_ok};
use hyper::server::Server;
use hyper::rt::Future;
use super::router::Router;
use super::builder::BuilderWithHandlers;

#[derive(Debug, Clone)]
pub struct ServerOptions {
    pub port: u16,
    pub cache_max_age: i32,
    pub ip: IpAddr,
}
impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            // cache 3 days
            cache_max_age: 24 * 3600 * 3,
            port: 7070,
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }
}

pub fn serve_http(build: BuilderWithHandlers, options: ServerOptions) {
    let addr = SocketAddr::new(options.ip, options.port);
    
    let service = make_service_fn(move |_: &AddrStream| {
        let router = Router::new(build.clone(), options.clone());
        service_fn_ok(move |req: Request<Body>| {
            match router.route(req) {
                Ok(router_response) => router_response.response(),
                Err(error) => {
                    eprintln!("service error: {:?}", error);
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    response
                }
            }
        })
    });

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running on: {}", addr);

    hyper::rt::run(server)
}

pub fn serve_serverless(
    req: now_lambda::Request, build: BuilderWithHandlers, options: ServerOptions
) -> Result<impl now_lambda::IntoResponse, now_lambda::error::NowError> {
    let router = Router::new(build, options);
    match router.route(req) {
        Ok(router_response) => Ok(router_response),
        Err(error) => {
            let error_message = format!("service error: {:?}", error);
            eprintln!("{}", error_message);
            Err(now_lambda::error::NowError::new(&error_message))
        }
    }
}
