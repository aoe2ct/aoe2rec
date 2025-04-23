from dataclasses import dataclass
from datetime import timedelta

try:
    from typing import override
except ImportError:
    def override(f):
        return f

@dataclass(slots=True)
class Chat:
    """Class representation of a chat message.

    Attributes:
        timestamp (timedelta): The time at which the message was sent, 
                               relative to the start of the game.
        player (str): The player who sent the message.
        message (str): The content of the message.                               
    """
    timestamp: timedelta
    player: str
    message: str

    @override
    def __str__(self):
        return f"{self.timestamp} - {self.player}: {self.message}"
