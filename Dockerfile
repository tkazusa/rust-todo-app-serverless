FROM public.ecr.aws/lambda/provided:al2 AS builder
ENV PACKAGE_NAME todo 

WORKDIR /todo

# リンカーとしてgccを利用する、openssl をインストールする
RUN yum install -y gcc openssl-devel

# rustupでRustツールチェーンをインストールする
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH $PATH:/root/.cargo/bin
RUN rustup install stable

# 更新頻度が低い Cargo.toml は序盤で Copy
COPY Cargo.toml Cargo.toml
RUN mkdir src
# Cargo.toml の内容だけ build しておきたいから、からの main.rs を作成
RUN echo "fn main(){}" > src/main.rs
# その上で、cargo build する
RUN cargo build --release

# 変更されやすい ./src 以下はこのタイミングで Copy & build
COPY ./src ./src
COPY ./templates ./templates
RUN rm -f target/release/deps/todo*
RUN cargo build --release

# AWS Lambda 用に提供されているイメージを活用して、push するためのイメージを作成
FROM public.ecr.aws/lambda/provided:al2

# 実行ファイルを起動するようにするため、ファイル名を "bootstrap" に変更する
# COPY ./target/release/${PACKAGE_NAME} ${LAMBDA_RUNTIME_DIR}/bootstrap
COPY --from=builder /todo/target/release/todo ${LAMBDA_RUNTIME_DIR}/bootstrap

# カスタムランタイム同様ハンドラ名は利用しないため、適当な文字列を指定する。
CMD [ "lambda-handler" ]