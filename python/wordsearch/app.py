from PySide6.QtWidgets import QApplication

from wordsearch.window import MainWindow
from wordsearch.event_manager import AppState, EventManager

import json
import subprocess
import tempfile
import asyncio

class App:
    qapp: QApplication
    window: MainWindow
    state: AppState
    event_manager: EventManager
    board: list

    def __init__(self, argv):
        self.qapp = QApplication(argv)
        self.event_manager = EventManager()
        self.window = MainWindow(self.qapp, self.event_manager)
        self.window.show()

        self.state = AppState.READY
        self.board = []

        self.window.generate_signal.connect(lambda: asyncio.run(self.generate_board()))

    def exec(self) -> int:
        return self.qapp.exec()

    async def generate_board(self):
        self.state = AppState.GENERATING
        self.event_manager.state_changed.emit(self.state)

        words = self.window.get_words()

        with tempfile.NamedTemporaryFile(mode="w+t", encoding="utf-8") as word_list_file:
            word_list_file.write(words)
            word_list_file.flush()

            process = await asyncio.create_subprocess_exec("./bin/wordsearch", word_list_file.name, "--json",
                                                           stdout=asyncio.subprocess.PIPE,
                                                           stderr=asyncio.subprocess.PIPE)
            assert(process.stdout and process.stderr)
            output = await process.stdout.read()
            errors = await process.stderr.read()
            if errors:
                print(f"STDERR: {errors.decode().strip()}")

            await process.wait()
            if process.returncode != 0:
                print(f"Process exited with nonzero exit code: {process.returncode}")

            self.board = json.loads(output.decode("ascii"))
            print(self.board)

        self.state = AppState.READY
        self.event_manager.state_changed.emit(self.state)
