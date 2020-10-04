#[macro_use]
extern crate log;

use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, RequestExt, Response};
use std::fs::read_dir;
use git2::{Repository, Reference, Tree, ObjectType};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    lambda::run(handler(handle_index)).await?;
    Ok(())
}

async fn handle_index(r: Request, _: Context) -> Result<Response<Body>, Error> {
    let repo = Repository::open_bare("/opt/wikiquotes-ludzie")?;
    let mut first_ref: Option<Reference> = None;
    let mut refs = repo.references()?;
    if let Some(r) = refs.by_ref().next() {
        first_ref = r.ok();
    }
    let first_ref = first_ref.map(|r| r.target().unwrap());

    let mut tree: Option<Tree> = None;
    if let Some(commit) = first_ref {
        let commit_obj1 = repo.revparse_single(&commit.to_string())?;
        let commit_obj = commit_obj1.as_commit().unwrap();
        tree = commit_obj.tree().ok();
    }

    let mut ls_result = vec![];
    let id_param = r.path_parameters().get("id").unwrap_or("").to_string();
    ls_result.push(id_param);
    if let Some(tree) = tree {
        for entry in tree.iter() {
            let name = entry.name().unwrap();
            let kind = entry.kind().unwrap();
            if kind == ObjectType::Tree {
                ls_result.push(format!("[{}],{}", name, entry.id().to_string()))
            } else {
                ls_result.push(format!("{},{}", name, entry.id().to_string()))
            }
        }
    };

    let j = ls_result.join("\n");
    Ok(j.into_response())
}
