from aws_cdk import (
    core, 
    aws_dynamodb as ddb
    )

from app.entities import Base
from app.props.dynamodb import (
    DynamoDB
)

class DynamoDBBase(Base):
    """DynamoDB基底class"""

    dynamodb: DynamoDB


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