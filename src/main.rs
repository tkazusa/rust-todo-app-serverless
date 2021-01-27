use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}

impl ResponseError for MyError {}

// MyError は actix_web::ResponseError を実装しているので、
// index の戻り値に MyError を使うことが出来ます。
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";
    // HttpResponse::Ok() はステータスコード 200 を持つ HttpResponseBuilder という構造体を返す。
    // HttpResponseBuilder の　body() という暗数にレスポンスのボディを渡すと HttpResponse が返ってくる
    // 関数の型が Result なので、返り値は Ok 型
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())

}