from aws_cdk import (
    core, 
    aws_dynamodb as ddb
    )

from src.entities.base import Base
from src.props.dynamodb import (
    DynamoDB
)

class DynamoDBBase(Base):
    """DynamoDB基底class"""

    lambda_fn: LambdaFn


class DynamoDB(DynamoDBBase):
    """AWS Lambdaタスク定義"""

    dynamodb = DynamoDB(
        id = "todo-app-on-lambda-dynamodb",
        table_name = "rust-todo",
        partition_key= ddb.Attribute(
            name= "id",
            type= ddb.AttributeType.STRING
            ),
        removal_policy = core.RemovalPolicy.DESTROY
        )