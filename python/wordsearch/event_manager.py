from enum import Enum
from PySide6.QtCore import QObject, Signal

class AppState(Enum):
    READY = 1
    GENERATING = 2
    GENERATED = 3


class EventManager(QObject):
    state_changed = Signal(AppState)
