use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

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

// ToDo のリストを持つ構造体
//#[template(path = "index.html")] で index.html と紐付けられている
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl ResponseError for MyError {}

// MyError は actix_web::ResponseError を実装しているので、
// index の戻り値に MyError を使うことが出来ます。
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();

    let client = DynamoDbClient::new(Region::ApNortheast1);
    let items_vector = scan(client).unwrap().items.unwrap();
    
    let client = DynamoDbClient::new(Region::ApNortheast1);
    let test = scan(client);
    println!("{:?}", test);

    for item in items_vector.iter(){
        entries.push(TodoEntry{
            id: item["id"].s.as_ref().unwrap().to_string(),
            text: item["text"].s.as_ref().unwrap().to_string()
        })};

    // IndexTemplate は Template から derive してる、多分 html.render で HTML を生成
    let html = IndexTemplate { entries };
    
    // ? は Result 型を返関数の中で使われる。値がOkなら中の値を、Err なら Err になったタイミングの値を返す
    let response_body = html.render()?;
    // HttpResponse::Ok() はステータスコード 200 を持つ HttpResponseBuilder という構造体を返す。
    // HttpResponseBuilder の　body() という暗数にレスポンスのボディを渡すと HttpResponse が返ってくる
    // 関数の型が Result なので、返り値は Ok 型
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

// async キーワードがついているから、非同期関数となる。返り値は Future<Output> 型となる
// この中では await キーワードを使って待機させる
// 非同期タスクの完了には、ランタイムを準備して、その中で、実行。
// actic_rt が tokio のシングルスレッドの非同期ランタイムを用意してくれている。
#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())

}

// actix_web のテストについては actic_web の testing の項目を参考い
// https://actix.rs/docs/testing/
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}