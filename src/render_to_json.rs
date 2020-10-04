use crate::data::PageOrCategory;
use lambda_http::{Response, Body};
use serde_json::Value;

pub fn render(p: &PageOrCategory) -> Response<Body> {
    let v = serde_json::to_value(p).unwrap();
    Response::builder()
        .header("content-type", "application/json; charset=utf-8")
        .body(Body::from(v.to_string()))
        .unwrap()
}