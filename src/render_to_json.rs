use crate::data::{PageOrCategory, Cite};
use lambda_http::{Response, Body};
use serde_json::Value;

fn render(val: Value) -> Response<Body> {
    Response::builder()
        .header("content-type", "application/json; charset=utf-8")
        .body(Body::from(val.to_string()))
        .unwrap()
}

pub fn render_page(p: &PageOrCategory) -> Response<Body> {
    render(serde_json::to_value(p).unwrap())
}

pub fn render_cite(c: &Cite) -> Response<Body> {
    render(serde_json::to_value(c).unwrap())
}