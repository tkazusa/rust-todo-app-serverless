use tokio;
use std::collections::HashMap;

use rusoto_core::{Region, RusotoError};
// 今回紹介する①アイテム登録(PutItemInput) ②アイテム取得(GetItemInput) ③アイテム削除(DeleteItemInput)で使用するstructのみを宣言しています
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient,
    PutItemInput, PutItemOutput, PutItemError,
    ScanInput, ScanOutput, ScanError, AttributeValue};
pub struct TodoEntry {
    id: String,
    text: String,
}

#[tokio::main(flavor = "current_thread")]
pub async fn add_task(todoentry: TodoEntry) -> Result<PutItemOutput, RusotoError<PutItemError>>{
    let mut create_key: HashMap<String, AttributeValue> = HashMap::new();
    // HashMapのkeyにはパーティションキーで指定した文字列を
    // valueにはLambdaコール時に受け渡されるイベント引数を指定します
    // HashMap への key-value は insert で一つづつ追加
    create_key.insert(String::from("id"), AttributeValue {
        s: Some(String::from(todoentry.id)),
        ..Default::default()
    });

   create_key.insert(String::from("text"), AttributeValue {
        s: Some(String::from(todoentry.text)),
        ..Default::default()
    });

    let create_serials = PutItemInput {
        item: create_key,
        table_name: String::from("rust-todo"),
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::ApNortheast1);
    client.put_item(create_serials).await
    
}

// runtime に tokio を使うことを宣言
#[tokio::main(flavor = "current_thread")]
pub async fn scan(client: DynamoDbClient) -> Result<ScanOutput, RusotoError<ScanError>> {
    let scan_input = ScanInput {
        table_name: String::from("rust-todo"),
        // 
        limit: Some(10),
        ..Default::default()
    };
    client.scan(scan_input).await
}


#[cfg(test)]
mod test {
    use super::*;
    use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher, MockResponseReader, ReadMockResponse};

    #[test]
    fn scan_todoentories_in_dynamodb() {
        let body = MockResponseReader::read_response(
            "test_resorces",
            "dynamodb_scan_response.json",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&body);
        let client = DynamoDbClient::new_with(mock, MockCredentialsProvider, Region::ApNortheast1);
        let item_vector = scan(client).unwrap().items.unwrap();
        
        let test_id = item_vector[0]["id"].s.as_ref().unwrap().to_string();
        assert_eq!("test", test_id);
        
        let test_text = item_vector[0]["text"].s.as_ref().unwrap().to_string();
        assert_eq!("hello world", test_text);
    }
}