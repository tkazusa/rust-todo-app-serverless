use std::collections::HashMap;

use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient,
    PutItemInput, PutItemOutput,
    ScanInput, ScanOutput, AttributeValue};
pub struct TodoEntry {
    pub id: String,
    pub text: String,
}

pub async fn scan(client: &DynamoDbClient) -> ScanOutput {
    let scan_input = ScanInput {
        table_name: String::from("rust-todo"),
        limit: Some(10),
        ..Default::default()
    };
    client.scan(scan_input).await.unwrap()
}

pub async fn add(client: &DynamoDbClient, todoentry: TodoEntry) -> PutItemOutput {
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
    client.put_item(create_serials).await.unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher, MockResponseReader, ReadMockResponse};

    #[tokio::test]
    async fn scan_todoentories_in_dynamodb() {
        let body = MockResponseReader::read_response(
            "test_resources",
            "dynamodb_scan_response.json",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&body);
        let client = DynamoDbClient::new_with(mock, MockCredentialsProvider, Region::ApNortheast1);
        let item_vector = scan(&client).await.items.unwrap();
        
        let test_id = item_vector[0]["id"].s.as_ref().unwrap().to_string();
        assert_eq!("test", test_id);
        
        let test_text = item_vector[0]["text"].s.as_ref().unwrap().to_string();
        assert_eq!("hello world", test_text);
    }
    
}
