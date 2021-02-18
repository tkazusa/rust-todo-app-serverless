from app.entities import Base
from app.props.api_gateway import (
    APIGateway
)

class APIGatewayBase(Base):
    """DynamoDB基底class"""

    api_gateway: APIGateway


class APIGateway(APIGatewayBase):
    """AWS Lambdaタスク定義"""

    api_gateway = APIGateway(
        id = "todo-app-on-lambda-apigateway",
        api_name = "HttpGateway")