FROM rust:1.49 AS builder

WORKDIR /todo

# 更新頻度が低い Cargo.toml は序盤で Copy
COPY Cargo.toml Cargo.toml
RUN mkdir src
# Cargo.toml の内容だけ build しておきたいから、からの main.rs を作成
RUN echo "fn main(){}" > src/main.rs
# その上で、cargo build する
RUN cargo build --release

# 変更サれやすい ./src 以下はこのタイミングで Copy & build
COPY ./src ./src
COPY ./templates ./templates
RUN rm -f target/release/deps/todo*
RUN cargo build --release

# リリース用イメージには debian を使用
FROM debian:10.4

COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
CMD ["bash"]