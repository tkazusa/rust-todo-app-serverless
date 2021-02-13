#!/usr/bin/env python3

from aws_cdk import core
from app.stack.todo_app_on_lambda_stack import TodoAppLambdaStack


app = core.App()

# 全てのリソースに設定するタグ
tags = {'CreatedBy': 'tkazusa'}

TodoAppLambdaStack(
    app,
    id='todo-app-on-lambda-stack',
    tags=tags)

app.synth(skip_validation=False)