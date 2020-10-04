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

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

impl Cite {
    pub fn from(s: &str) -> Result<Cite,Error> {
        let mut meta = vec![];
        let mut body_lines = vec![];

        let mut in_head = true;
        for line in s.lines() {
            if line.is_empty() {
                in_head = false;
            } else {
                if in_head {
                    let spl: Vec<&str> = line.splitn(2, ':').collect();
                    let m = if spl.len() == 1 {
                        Meta { key: "".to_string(), value: spl[0].to_string() }
                    } else {
                        Meta { key: spl[0].to_string(), value: spl[1].to_string() }
                    };
                    meta.push(m);
                } else {
                    body_lines.push(line)
                }
            }
        }

        Ok(Cite {
            text: body_lines.join("\n"),
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