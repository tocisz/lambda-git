#[macro_use]
extern crate log;

use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, RequestExt, Response};
use std::fs::read_dir;

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
    let mut entries = read_dir("/opt/polacy")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    let mut ps = vec![];
    for p in entries {
        ps.push(String::from(p.to_str().unwrap_or("")))
    }
    let j = ps.join(", ");
    Ok(j.into_response())
}
