use git2::{Tree, ObjectType, Repository, Oid};
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
    metadata: Vec<Meta>,
    link: String
}

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

impl Cite {
    pub fn from(repo: &Repository, id: Oid, s: &str) -> Result<Cite,Error> {
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
                        Meta { key: "".to_string(), value: spl[0].trim().to_string() }
                    } else {
                        Meta { key: spl[0].trim().to_string(), value: spl[1].trim().to_string() }
                    };
                    meta.push(m);
                } else {
                    body_lines.push(line)
                }
            }
        }

        Ok(Cite {
            text: body_lines.join("\n"),
            metadata: meta,
            link: short_id(repo, id)?
        })
    }
}

#[derive(Serialize)]
pub struct Meta {
    key: String,
    value: String
}

pub fn parse_tree(r: &Repository, tree: &Tree) -> Result<PageOrCategory,Error> {
    let mut trees = vec![];
    let mut blobs = vec![];
    let mut is_category = false;
    let mut is_page = false;
    let mut name = None;
    for entry in tree.iter() {
        let entry_name = entry.name().unwrap_or("").to_string();
        let kind = entry.kind().unwrap_or(ObjectType::Any);
        if kind == ObjectType::Tree {
            trees.push((entry_name, entry.id()));
        } else if kind == ObjectType::Blob {
            if entry_name == "cat.txt" {
                is_category = true;
                name = Some(get_blob_contents(r, entry.id())?)
            } else if entry_name == "art.txt" {
                is_page = true;
                name = Some(get_blob_contents(r, entry.id())?)
            }
            blobs.push((entry_name, entry.id()));
        }
    }
    if is_category {
        let name = name.unwrap_or_else(|| "".to_string());
        Ok(PageOrCategory::Category(
            Category::new(r, name, trees)
        ))
    } else if is_page {
        let name = name.unwrap_or_else(|| "".to_string());
        Ok(PageOrCategory::Page(
            Page::new(r, name, blobs)
        ))
    } else {
        Err(Error::from("Wrong tree"))
    }
}

fn short_id(repo: &Repository, i: Oid) -> Result<String, git2::Error> {
    repo.find_object(i, None).and_then(|o| {
        o.short_id().and_then(|o| {
            o.as_str().map(|s|{
                s.to_string()
            }).ok_or_else(|| git2::Error::from_str("can't unwrap short hash"))
        })
    })
}

impl Category {
    fn new(repo: &Repository, name: String, trees: Vec<(String,Oid)>) -> Self {
        let links = trees.into_iter().map(|(title, i)|{
            short_id(repo, i).map(|href| Link {title, href})
        }).filter_map(|x|{ // log and ignore errors
            match x {
                Err(e) => {
                    warn!("Error when getting page link {}", e);
                    None
                },
                Ok(o) => Some(o)
            }
        }).collect();
        Category { name, links }
    }
}

impl Page {
    fn new(repo: &Repository, name: String, blobs: Vec<(String,Oid)>) -> Self {
        let cites = blobs.into_iter().map(|(entry_name, i)| {
            if entry_name == "art.txt" {
                Ok(None)
            } else {
                get_blob_contents(repo, i).map(|s|{
                    Cite::from(repo, i, &s).ok()
                })
            }
        }).filter_map(|x|{ // log and ignore errors
            match x {
                Err(e) => {
                    warn!("Error when getting cite blob {}", e);
                    None
                },
                Ok(o) => o
            }
        }).collect();
        Page { name, cites }
    }
}

fn get_blob_contents(repo: &Repository, id: Oid) -> Result<String,Error> {
    Ok(std::str::from_utf8(repo.find_blob(id)?.content())?.to_string())
}