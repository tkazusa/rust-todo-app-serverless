use lamedh_runtime::{Context, handler_fn, run, Error};
use serde_json::{json, Value};

use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDbClient
};

mod dynamodb_operations;
use crate::dynamodb_operations::scan;

struct TodoEntry {
    id: String,
    text: String,
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

    Ok(json!(format!("Hello {} from Rust Container on AWS Lambda!", entries[0].id)))
}
