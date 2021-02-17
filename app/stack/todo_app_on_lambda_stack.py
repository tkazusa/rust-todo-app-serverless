import os
from typing import Any

from aws_cdk import (
    core,
    aws_lambda,
    aws_ecr,
    aws_dynamodb,
    aws_apigatewayv2,
    aws_apigatewayv2_integrations
)  

class TodoAppLambdaStack(core.Stack):

    def __init__(
            self,
            scope: core.Construct,
            id: str,
            lambda_fn_entity,
            dynamodb_entity,
            api_gateway_entity,
            **kwargs) -> None:
        super().__init__(scope, id, **kwargs)


        # ====================================
        # Amazn ECR
        # ====================================
        ## Create new Container Image.
        ecr_image = aws_lambda.EcrImageCode.from_asset_image(
            directory = os.path.join("./", "lambda-image")
        )

        # ====================================
        # Amazon DynamoDB
        # ====================================
        ddb_table = aws_dynamodb.Table(self, 
            **dynamodb_entity.dynamodb.to_dict()
        )

        # ====================================
        # AWS Lambda
        # ====================================
        handler = aws_lambda.Function(
            self,
            code = ecr_image,
            **lambda_fn_entity.lambda_fn.to_dict()
        )

        ddb_table.grant_read_data(handler)
        ddb_table.grant_write_data(handler)

        # ====================================
        # Amazon API Gateway
        # ====================================
        api = aws_apigatewayv2.HttpApi(self,
            **api_gateway_entity.api_gateway.to_dict()
        )

        lambda_integration = aws_apigatewayv2_integrations.LambdaProxyIntegration(handler=handler)

        api.add_routes(
             path="/",
             methods=[aws_apigatewayv2.HttpMethod("ANY")],
             integration=lambda_integration
        )

        api.add_routes(
             path="/add",
             methods=[aws_apigatewayv2.HttpMethod("ANY")],
             integration=lambda_integration
        )

        api.add_routes(
             path="/delete",
             methods=[aws_apigatewayv2.HttpMethod("ANY")],
             integration=lambda_integration
        )