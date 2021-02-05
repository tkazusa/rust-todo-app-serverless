# AWS Lambda 用に提供されているイメージを活用
FROM public.ecr.aws/lambda/provided:al2

# 実行ファイルを起動するようにするため、ファイル名を "bootstrap" に変更する
COPY ./target/release/hello-container ${LAMBDA_RUNTIME_DIR}/bootstrap

# カスタムランタイム同様ハンドラ名は利用しないため、適当な文字列を指定する。
CMD [ "lambda-handler" ]