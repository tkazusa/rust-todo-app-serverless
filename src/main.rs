use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;
use tokio;
use std::collections::HashMap;

use rusoto_core::Region;
// 今回紹介する①アイテム登録(PutItemInput) ②アイテム取得(GetItemInput) ③アイテム削除(DeleteItemInput)で使用するstructのみを宣言しています
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, ScanInput, ListTablesInput, DeleteItemInput, AttributeValue};
struct TodoEntry {
    id: u32,
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


#[tokio::main]
async fn main() {
    let client = DynamoDbClient::new(Region::ApNortheast1);
    let list_tables_input: ListTablesInput = Default::default();
    
    let mut create_key: HashMap<String, AttributeValue> = HashMap::new();
    // HashMapのkeyにはパーティションキーで指定した文字列を
    // valueにはLambdaコール時に受け渡されるイベント引数を指定します
    // HashMap への key-value は insert で一つづつ追加
    create_key.insert(String::from("id"), AttributeValue {
        s: Some(String::from("test")),
        ..Default::default()
    });

   create_key.insert(String::from("text"), AttributeValue {
        s: Some(String::from("hello world")),
        ..Default::default()
    });

    let create_serials = PutItemInput {
        item: create_key,
        table_name: String::from("rust-todo"),
        ..Default::default()
    };

    let scan_input = ScanInput {
        table_name: String::from("rust-todo"),
        // 
        limit: Some(10),
        ..Default::default()
    };

    match client.scan(scan_input).await {
        Ok(result) => match result.items {
            Some(v) => for kv in v.iter() {
                println!("> {:?}", kv);
            
            }
            None => println!("None")
        },
        Err(error) => {
            panic!("Error: {:?}", error);
        },
    };

 
    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");
 
                for table_name in table_name_list {
                    println!("{}", table_name);
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

/* 
// MyError は actix_web::ResponseError を実装しているので、
// index の戻り値に MyError を使うことが出来ます。
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    
    // ダミーのデータ
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });

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
*/