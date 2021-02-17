from dataclasses import dataclass, field

from aws_cdk import (
    core, 
    aws_lambda as _lambda
    )

from .base import Base


@dataclass(frozen=True)
class LambdaFn(Base):
    id: str
    description: str
    code: _lambda.EcrImageCode
    handler: _lambda.Handler.FROM_IMAGE
    runtime: _lambda.Runtime.FROM_IMAGE
    function_name: str
    memory_size: int
    reserved_concurrent_executions: int
    timeout: core.Duration