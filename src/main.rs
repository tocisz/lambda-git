mod data;
mod render_to_json;

#[macro_use]
extern crate log;

use lambda_http::{handler, lambda, Body, Context, Request, RequestExt, Response};
use git2::{Repository, Reference, Tree, ObjectType, Oid};
use crate::data::Cite;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    lambda::run(handler(handle_index)).await?;
    Ok(())
}

fn get_commit(repo: &Repository) -> Result<Option<Oid>, Error> {
    let mut first_ref: Option<Reference> = None;
    let mut refs = repo.references()?;
    if let Some(r) = refs.by_ref().next() {
        first_ref = r.ok();
    }
    let first_ref = first_ref.map(|r| r.target().unwrap());
    Ok(first_ref)
}

fn get_root_tree(repo: &Repository, commit: Option<Oid>) -> Result<Option<Tree>, Error> {
    let mut tree: Option<Tree> = None;
    if let Some(commit) = commit {
        let commit_obj1 = repo.revparse_single(&commit.to_string())?;
        let commit_obj = commit_obj1.as_commit().unwrap();
        tree = commit_obj.tree().ok();
    }
    Ok(tree)
}

/*
fn list_tree(tree: Tree) -> Vec<String> {
    let mut ls_result = vec![];
    for entry in tree.iter() {
        let name = entry.name().unwrap_or("");
        let kind = entry.kind().unwrap_or(ObjectType::Any);
        if kind == ObjectType::Tree {
            ls_result.push(format!("[{}],{}", name, entry.id().to_string()))
        } else {
            ls_result.push(format!("{},{}", name, entry.id().to_string()))
        }
    }
    ls_result
}

fn txt_response(s: &str) -> Result<Response<Body>, Error> {
    Ok(Response::builder().header("content-type", "text/plain; charset=utf-8").body(Body::from(s))?)
}
*/

async fn handle_index(req: Request, _: Context) -> Result<Response<Body>, Error> {
    debug!("Request is {} {}", req.method(), req.uri().path());
    let repo = Repository::open_bare("/opt/wikiquotes-ludzie")?;

    let tree;
    let blob;
    match req.path_parameters().get("id") {
        Some(id) => {
            let oid = Oid::from_str(id)?;
            let obj = repo.find_object(oid, None)?;
            let kind = obj.kind().unwrap_or(ObjectType::Any);
            match kind {
                ObjectType::Tree => {
                    tree = Some(repo.find_tree(oid)?);
                    blob = None;
                },
                ObjectType::Blob => {
                    blob = Some(repo.find_blob(oid)?);
                    tree = None;
                },
                _ => {
                    tree = None;
                    blob = None;
                }
            }
        },
        None => {
            let commit = get_commit(&repo)?;
            tree = get_root_tree(&repo, commit)?;
            blob = None;
        }
    }

    if let Some(t) = tree {
        let parsed = data::parse_tree(&repo,&t)?;
        debug!("Returning tree response.");
        Ok(render_to_json::render_page(&parsed))
    } else if let Some(b) = blob {
        let s = std::str::from_utf8(b.content())?;
        let cite = Cite::from(s)?;
        debug!("Returning blob response.");
        Ok(render_to_json::render_cite(&cite))
    } else {
        error!("Wrong object hash");
        Err(Error::from("Wrong object hash"))
    }
}
