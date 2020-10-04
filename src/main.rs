#[macro_use]
extern crate log;

use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, RequestExt, Response};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    lambda::run(handler(handle_index)).await?;
    Ok(())
}

// async fn route(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
//     debug!("Request is {} {}", req.method(), req.uri().path());
//     match req.uri().path() {
//         "/" => handle_index(req).await,
//         "/webhook" => handle_webhook(req).await,
//         _ => handle_404(req).await,
//     }
// }

async fn handle_index(_: Request, _: Context) -> Result<Response<Body>, Error> {
    Ok("Welcome!".into_response())
}
