use lamedh_runtime::{run};
use askama::Template;

use lamedh_http::{
    lambda::{Context, Error},
    IntoResponse, Request, Response, handler
 };
 

use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDbClient
};

mod dynamodb_operations;
use crate::dynamodb_operations::scan;

// ToDo のリストを持つ構造体
struct TodoEntry {
    id: String,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler(func)).await?;
    Ok(())
}

async fn func(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let mut entries = Vec::new();

    let client = DynamoDbClient::new(Region::ApNortheast1);
    let items_vector = scan(client).await.items.unwrap();

    for item in items_vector.iter(){
        entries.push(TodoEntry{
            id: item["id"].s.as_ref().unwrap().to_string(),
            text: item["text"].s.as_ref().unwrap().to_string()
        })};

    // IndexTemplate は Template から derive してる、多分 html.render で HTML を生成
    let html = IndexTemplate { entries };
    let response_body = html.render()?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(response_body)
        .expect("failed to render response")) 
}
