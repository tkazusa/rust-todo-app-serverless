# Rust ToDo app on AWS serverless sevices

## Architecture

<img src="img/architecture.png" width="480px">

- Amazon API Gateway をエンドポイントとし、AWS Lambda 上に Todo の閲覧、追加、削除　を操作する API を実装。
- クライアントからのアクセスに対し、AWS Lambda 上で Rust のテンプレートエンジン `askama` を使って HTML を生成し、レスポンスする。
- AWS Lambda へリクエストされたタイミングで Basic 認証を実施。

## Requirements

- Python>=3.8.5
- npm>=6.14
- aws-cdk>=1.89.0

## Deploy

AWS CDK を用いてデプロイする場合。

```bash
cdk synth
cdk deploy
```

既に Amazon ECR や AWS Lambda などがデプロイされていて、 Docker コンテナを AWS Lambda にアップデートしたい場合。

```bash
REGION=XXXX
AWS_ACCOUNT_ID=XXXX
cd lambda-image
# イメージをビルド
docker build -t todo-app .
# Amazon ECR リポジトリへログイン
aws ecr get-login-password --region $REGION | sudo docker login --username AWS --password-stdin $AWS_ACCOUNT_ID.dkr.ecr.ap-northeast-1.amazonaws.com
# Amazon ECR へ push できるようにコンテナへタグ付け
docker image tag todo-app:latest $AWS_ACCOUNT_ID.dkr.ecr.$REGION.amazonaws.com/todo-app:latest
docker image push $AWS_ACCCOUNT_ID.dkr.ecr.$REGION.amazonaws.com/todo-app:latest
aws lambda update-function-code --function-name todo-app-container --image-uri $AWS_ACCOUNT_ID.dkr.ecr.$REGION.amazonaws.com/todo-app:latest
```

## Comments

### AWS Lambda への Rust コンテナのデプロイについて

Rust での AWS Lambda ランタイムについては、awslabs が監理している [`aws-lambda-rust-runtime`](https://github.com/awslabs/aws-lambda-rust-runtime) はあまりメンテナンスされていないので、`lamedh-dev` がフォークしてきた、[aws-lambda-rust-runtime](https://github.com/lamedh-dev/aws-lambda-rust-runtime) を活用している。

### AWS Lambda からのレスポンスについて

[HTTP API の AWS Lambda プロキシ統合の使用](https://docs.aws.amazon.com/ja_jp/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html)にあるように、レスポンスの body だけを記載する場合、下記のように `Content-Type` が `application/json` に指定されてしまう。

```
{
  "isBase64Encoded": false,
  "statusCode": 200,
  "body": "Hello from Lambda!",
  "headers": {
    "Content-Type": "application/json"
  }
}
```

今回のように、ヘッダを変更して、`text/html; charset=UTF-8` などを指定したい場合、

```
{
    "cookies" : ["cookie1", "cookie2"],
    "isBase64Encoded": true|false,
    "statusCode": httpStatusCode,
    "headers": { "headerName": "headerValue", ... },
    "body": "Hello from Lambda!"
}   
```

といった形のレスポンスを作成する必要がある。今回は `lamedh_http` クレートを使用した。

## References

- [AWS Lambda Rust docker builder](https://github.com/softprops/lambda-rust): AWS Lambda 向けにビルドするための Docker イメージを提供。しかし、このままビルドすると OPENSSL_DIR 関連のエラーが発生する。`OpenSSL-devel` の追加などが必要では。
- [Rustのasync/awaitをスムーズに使うためのテクニック](https://qiita.com/qnighy/items/59133e69a0ba0c6a7fef)
- [awslabs/dynein](https://github.com/awslabs/dynein): AWSLabs が提供している DynamoDB のための Rust 製 CLI。Rust の書き方やテストの方法など参考にしたい。 
- GitHub Actions の環境には AWS CLI は pre-install