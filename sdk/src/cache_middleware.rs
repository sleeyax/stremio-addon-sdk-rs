use tide::utils::BoxFuture;
use tide::middleware::{Middleware, Next};
use tide::{Request, Response};

pub struct Cache {
    max_age: i32
}
impl Cache {
    pub fn new(max_age: i32) -> Self {
        Self {
            max_age: max_age
        }
    }
}

// NOTE: BoxFuture<'a, Response> is a futuress 0.3.x feature.
// stremio-core uses 0.1.x, so as a work around we use a modified version of tide.

impl<State: Send + Sync + 'static> Middleware<State> for Cache {
    fn handle<'a>(&'a self, req: Request<State>, next: Next<'a, State>) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            let res: Response = next.run(req).await;
            res.set_header("Cache-Control", format!("max-age={}, public", self.max_age))
        })
    }
}

