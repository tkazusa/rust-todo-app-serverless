# Rust ToDo app on AWS serverless sevices

## requirements
- rust 1.49.0
- httpie 0.9.8

## ローカルでの実行
```
$ docker build -t todo-app .
$ docker run -p 8080:8080 todo-app
$ http localhost:8080
```