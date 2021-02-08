use lambda_http::{lambda, IntoResponse, Request, RequestExt, Response, Body};
use lamedh_runtime::{Context, handler_fn, run, Error};
use serde_json::{json, Value};
use askama::Template;

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

//#[template(path = "index.html")] で index.html と紐付けられている
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler_fn(func)).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
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

    //Ok(json!(format!("Hello {} from Rust Container on AWS Lambda!", entries[0].id)))
    Ok(
        json!(
            {"headers":
                {"Content-Type": "text/html"},
                "body": response_body
            }
        )
    )
}
