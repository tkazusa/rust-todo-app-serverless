#!/usr/bin/env python3

from aws_cdk import core

from app.entities.lambda_fn import LambdaFn
from app.entities.dynamodb import DynamoDB
from app.entities.api_gateway import APIGateway
from app.stack.todo_app_on_lambda_stack import TodoAppLambdaStack

app = core.App()

# 全てのリソースに設定するタグ
tags = {'CreatedBy': 'tkazusa'}

TodoAppLambdaStack(
    app,
    id='todo-app-on-lambda-stack',
    lambda_fn_entity=LambdaFn,
    dynamodb_entity=DynamoDB,
    api_gateway_entity=APIGateway,
    tags=tags)

app.synth(skip_validation=False)
