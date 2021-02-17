from dataclasses import dataclass, field

from aws_cdk import (
    core, 
    aws_dynamodb as ddb
    )

from .base import Base


@dataclass(frozen=True)
class DynamoDB(Base):
    id: str
    table_name: str
    partition_key: ddb.Attribute
    removal_policy: core.RemovalPolicy