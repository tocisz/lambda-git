// #[macro_use]
// extern crate log;

use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, Response};
use git2::{Repository, Reference, Tree, ObjectType, Oid};

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

fn get_tree(repo: &Repository, commit: Option<Oid>) -> Result<Option<Tree>, Error> {
    let mut tree: Option<Tree> = None;
    if let Some(commit) = commit {
        let commit_obj1 = repo.revparse_single(&commit.to_string())?;
        let commit_obj = commit_obj1.as_commit().unwrap();
        tree = commit_obj.tree().ok();
    }
    Ok(tree)
}

fn list_tree(tree: Option<Tree>) -> Vec<String> {
    let mut ls_result = vec![];
    if let Some(tree) = tree {
        for entry in tree.iter() {
            let name = entry.name().unwrap_or("");
            let kind = entry.kind().unwrap_or(ObjectType::Any);
            if kind == ObjectType::Tree {
                ls_result.push(format!("[{}],{}", name, entry.id().to_string()))
            } else {
                ls_result.push(format!("{},{}", name, entry.id().to_string()))
            }
        }
    };
    ls_result
}

async fn handle_index(_r: Request, _: Context) -> Result<Response<Body>, Error> {
    let repo = Repository::open_bare("/opt/wikiquotes-ludzie")?;

    let commit = get_commit(&repo)?;
    let tree = get_tree(&repo, commit)?;
    let ls_result = list_tree(tree);
    let j = ls_result.join("\n");
    Ok(j.into_response())
}
