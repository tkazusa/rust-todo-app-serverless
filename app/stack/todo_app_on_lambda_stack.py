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
            **kwargs) -> None:
        super().__init__(scope, id, **kwargs)


        # ====================================
        # Amazn ECR
        # ====================================
        image_name    = "todo-app"
        image_version = "latest"

        ## If use_pre_existing_image is True
        ## then use an image that already exists in ECR.
        ## Otherwise, build a new image
        use_pre_existing_image = False

        if (use_pre_existing_image):

            ## Container was build previously, or elsewhere.
            ## Use the pre-existing container
            ecr_repository = aws_ecr.Repository.from_repository_attributes(self, 
                id              = "todo-app-on-lambda-ECR", 
                repository_arn  = '{0}.dkr.ecr.{1}.amazonaws.com/{2}:{3}'.format(core.Aws.ACCOUNT_ID, core.Aws.REGION, image_name, image_version),
                repository_name = image_name
            ) ## aws_ecr.Repository.from_repository_attributes

            ## Container Image.
            ## Pulled from the ECR repository.
            ecr_image = aws_lambda.EcrImageCode(
                repository = ecr_repository
            ) ## aws_lambda.EcrImageCode

        else:
            ## Create new Container Image.
            ecr_image = aws_lambda.EcrImageCode.from_asset_image(
                directory = os.path.join("./", "lambda-image")
            )

        # ====================================
        # Amazon DynamoDB
        # ====================================
        ddb_table = aws_dynamodb.Table(self, 
            id = "todo-app-on-lambda-dynamodb",
            table_name = "rust-todo",
            partition_key=aws_dynamodb.Attribute(
                name="id",
                type=aws_dynamodb.AttributeType.STRING
            ),
            removal_policy = core.RemovalPolicy.DESTROY
        )


        # ====================================
        # AWS Lambda
        # ====================================
        handler = aws_lambda.Function(self, 
            id = "todo-app-on-lambda",
            description   = "Rust Todo App on Lambda Container Function",
            code          =  ecr_image,
          ## Handler and Runtime must be *FROM_IMAGE*
          ## when provisioning Lambda from Container.
            handler       = aws_lambda.Handler.FROM_IMAGE,
            runtime       = aws_lambda.Runtime.FROM_IMAGE,
            # environment   = {"hello":"world"},
            function_name = "todo-app-on-lambda-container-function",
            memory_size   = 128,
            reserved_concurrent_executions = 10,
            timeout       = core.Duration.seconds(10),
        )

        ddb_table.grant_read_data(handler)
        ddb_table.grant_write_data(handler)
        

        # ====================================
        # Amazon API Gateway
        # ====================================
        api = aws_apigatewayv2.HttpApi(self,
            id = "todo-app-on-lambda-apigateway",
            api_name="HttpGateway")

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