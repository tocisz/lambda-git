mod data;
mod render_to_json;

#[macro_use]
extern crate log;

use lambda_http::{handler, lambda, Body, Context, Request, RequestExt, Response};
use git2::{Repository, Tree, ObjectType, Oid, Blob};
use crate::data::Cite;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    lambda::run(handler(handle_index)).await?;
    Ok(())
}

fn get_commit(repo: &Repository) -> Result<Option<Oid>, Error> {
    let result = if let Some(r) = repo.references()?.next() {
        Some(r.map(|r| r.target().unwrap())?)
    } else {
        None
    };
    Ok(result)
}

fn get_root_tree(repo: &Repository, commit_id: Option<Oid>) -> Result<Option<Tree>, Error> {
    let tree = if let Some(id) = commit_id {
        Some(repo.find_commit(id)?.tree()?)
    } else {
        None
    };
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

    match req.path_parameters().get("id") {
        Some(id) => {
            let obj = repo.revparse_single(id)?;
            let tree = obj.as_tree();
            let blob = obj.as_blob();
            let oid = obj.id();
            render(&repo, tree, blob, oid)
        },
        None => {
            let commit = get_commit(&repo)?;
            let tree = get_root_tree(&repo, commit)?;
            render(&repo, tree.as_ref(), None, Oid::zero())
        }
    }

}

fn render(repo: &Repository, tree: Option<&Tree>, blob: Option<&Blob>, oid: Oid) -> Result<Response<Body>, Error> {
    if let Some(t) = tree {
        let parsed = data::parse_tree(repo,&t)?;
        debug!("Returning tree response.");
        Ok(render_to_json::render_page(&parsed))
    } else if let Some(b) = blob {
        let s = std::str::from_utf8(b.content())?;
        let cite = Cite::from(oid, s)?;
        debug!("Returning blob response.");
        Ok(render_to_json::render_cite(&cite))
    } else {
        error!("Wrong object hash");
        Err(Error::from("Wrong object hash"))
    }
}
