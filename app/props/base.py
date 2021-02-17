from dataclasses import asdict, dataclass
from typing import Any, Dict


@dataclass(frozen=True)
class Base:
    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)