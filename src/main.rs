use askama::Template;
use lamedh_runtime::run;
use lamedh_http::{IntoResponse, Request, Response, handler, lambda::{Context, Error}};
 
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

mod dynamodb_operations;
use crate::dynamodb_operations::{scan, add, delete, TodoEntry, DeleteEntry};
use std::collections::HashMap;
use base64::encode;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Adapts a Handler to the lamedh_runtime::run interface
    // run(handler())の形で、request::LambdaRequest] をデシリアライズ
    run(handler(func)).await?;
    Ok(())
}

// lamedh_http::handler の要求する型は、 Result<Self::Response, Self::Error>>
async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let client = DynamoDbClient::new(Region::ApNortheast1);

    if check_authorization_header(&event) {
        let method = event.method().as_ref();
        let path = event.uri().path();

        if method == "POST" && path == "/add" {
            let item_id = scan(&client).await.items.unwrap().len() + 1;
            // "text=XXX" の形で body 部分が入力されてくるので、get(index=..5) で text= 以降をパースしている。
            let text = std::str::from_utf8(event.body().get(5.. ).unwrap()).unwrap();
            let todoentry = TodoEntry{id: item_id.to_string(), text: text.to_string()};
            let _putitemoutput = add(&client, todoentry).await;
        }

        if method == "POST" && path == "/delete" {
            let delete_id = std::str::from_utf8(event.body().get(3.. ).unwrap()).unwrap();
            let delete_entry = DeleteEntry{id: delete_id.to_string()};
            let _deleteitemoutput = delete(&client, delete_entry).await;
        }


        let mut entries = Vec::new();
        let items_vector = scan(&client).await.items.unwrap();

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

    } else {
        Ok(Response::builder()
            .status(401)
            .header("WWW-Authenticate", "Basic")
            .body("Unauthorized".to_string())
            .expect("failed to render response")) 
    }
}

fn check_authorization_header(event: &Request) -> bool {
    let mut accounts = HashMap::new();
    accounts.insert("user1", "pass1");
    
    let header_auth = event.headers().get("authorization");
    
    match header_auth {
        // header の中身が入っていた場合を評価
        Some(header_auth)  => {
            let auth_value: &str = header_auth.to_str().unwrap();
            // 登録されている user と pass のタプルで loop 
            for (user, pass) in accounts.iter(){
                let str = format!("{}:{}", user, pass);
                let encoded_value = encode(str);
                let check_value = format!("Basic {}", encoded_value);
                // 入力と登録されていたものが一致すれば true
                if &auth_value ==  &check_value {
                    return true;
                } else {
                    continue;
                }
            }
            // 一致したものがなければ false
            return false;
        }
        // そもそも autorization の中身が空なら false
        None => return false,
    }
}
