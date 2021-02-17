from dataclasses import dataclass, field

from .base import Base


@dataclass(frozen=True)
class APIGateway(Base):
    id: str
    api_name: str