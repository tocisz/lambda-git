use git2::Tree;
use serde::{Serialize};
use httparse::Status;

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

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

impl Cite {
    pub fn from(s: &str) -> Result<Cite,Error> {
        let mut meta = vec![];

        let mut headers = [httparse::EMPTY_HEADER; 64];
        let body;
        let result = httparse::parse_headers(s.as_bytes(), &mut headers)?;
        if let Status::Complete((pos, hrs)) = result {
            for h in hrs {
                meta.push(Meta{
                    key: h.name.to_string(),
                    value: std::str::from_utf8(h.value).unwrap().to_string()
                })
            }
            let dd = &s.as_bytes()[pos..];
            body = std::str::from_utf8(dd)?
        } else {
            body = s
        }

        Ok(Cite {
            text: body.to_string(),
            metadata: meta
        })
    }
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