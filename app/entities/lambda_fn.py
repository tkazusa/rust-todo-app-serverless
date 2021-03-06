from aws_cdk import (
    core,
    aws_lambda as _lambda
    )

from app.entities import Base
from app.props.lambda_fn import (
    LambdaFn
)



class LambdaFnBase(Base):
    """AWS Lambda基底class"""

    lambda_fn: LambdaFn


class LambdaFn(LambdaFnBase):
    """AWS Lambdaタスク定義"""

    lambda_fn = LambdaFn(
        id = "todo-app-on-lambda",
        description   = "Rust Todo App on Lambda Container Function",
        # code         =  ecr_image,
        ## Handler and Runtime must be *FROM_IMAGE*
        ## when provisioning Lambda from Container.
        handler       = _lambda.Handler.FROM_IMAGE,
        runtime       = _lambda.Runtime.FROM_IMAGE,
        # environment   = {"hello":"world"},
        function_name = "todo-app-on-lambda-container-function",
        memory_size   = 128,
        reserved_concurrent_executions = 10,
        timeout       = core.Duration.seconds(10),
        )