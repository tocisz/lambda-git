use git2::Tree;
use serde::{Serialize};

#[derive(Serialize)]
pub enum PageOrCategory {
    Page(Page),
    Category(Category)
}

#[derive(Serialize)]
pub struct Category {
    name: String,
    links: Vec<Link>
}

#[derive(Serialize)]
pub struct Link {
    title: String,
    href: String
}

#[derive(Serialize)]
pub struct Page {
    name: String,
    cites: Vec<Cite>
}

#[derive(Serialize)]
pub struct Cite {
    text: String,
    metadata: Vec<Meta>
}

#[derive(Serialize)]
pub struct Meta {
    key: String,
    value: String
}

pub fn parse_tree(t: &Tree) -> PageOrCategory {
    PageOrCategory::Page(Page{
        name: "Test".to_string(),
        cites: vec![]
    })
}