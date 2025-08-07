from PySide6.QtWidgets import QApplication
from wordsearch.options_dialog import PdfOptions

from wordsearch.window import MainWindow
from wordsearch.event_manager import AppState, EventManager

import json
import subprocess
import tempfile
import asyncio
import sys
import os

async def generate_pdf(words: str, output_filename: str, pdf_options: PdfOptions):
    with tempfile.NamedTemporaryFile(mode="w+t", encoding="utf-8") as word_list_file:
        word_list_file.write(words)
        word_list_file.flush()

        cli = ["./bin/wordsearch", word_list_file.name, "-o", output_filename]
        if pdf_options.grid_font_size:
            cli.extend(["--grid-font-size", str(pdf_options.grid_font_size)])
        if pdf_options.word_bank_font_size:
            cli.extend(["--word-bank-font-size", str(pdf_options.word_bank_font_size)])
        match pdf_options.page_size:
            case str(s):
                cli.extend(["--size", s])
            case (int(w), int(h)):
                cli.extend(["--size", str(w) + "," + str(h)])
        if pdf_options.margin:
            cli.extend(["--margin", str(pdf_options.margin)])
        if pdf_options.title:
            cli.extend(["--title", str(pdf_options.title)])
        if pdf_options.title_font_size:
            cli.extend(["--title-font-size", str(pdf_options.title_font_size)])
        if pdf_options.rows:
            cli.extend(["--rows", str(pdf_options.rows)])
        if pdf_options.cols:
            cli.extend(["--cols", str(pdf_options.cols)])

        process = await asyncio.create_subprocess_exec(*cli,
                                                       stdout=asyncio.subprocess.PIPE,
                                                       stderr=asyncio.subprocess.PIPE)
        assert(process.stdout and process.stderr)
        output = await process.stdout.read()
        errors = await process.stderr.read()
        if output:
            print(f"STDOUT: {output.decode().strip()}")
        if errors:
            print(f"STDERR: {errors.decode().strip()}")

        await process.wait()
        if process.returncode != 0:
            print(f"Process exited with nonzero exit code: {process.returncode}")

class App:
    qapp: QApplication
    window: MainWindow
    state: AppState
    event_manager: EventManager
    pdf_options: PdfOptions

    def __init__(self, argv):
        self.qapp = QApplication(argv)
        self.event_manager = EventManager()
        self.window = MainWindow(self.qapp, self.event_manager)
        self.window.show()

        self.state = AppState.READY
        self.pdf_options = PdfOptions()

        self.window.generate_signal.connect(lambda filename, open: asyncio.run(self.generate_board(filename, open)))
        self.window.pdf_options_changed.connect(self.pdf_options_changed)

    def exec(self) -> int:
        return self.qapp.exec()

    async def generate_board(self, filename: str, open_pdf: bool):
        self.state = AppState.GENERATING
        self.event_manager.state_changed.emit(self.state)

        await generate_pdf(self.window.get_words(), filename, self.pdf_options)

        self.state = AppState.GENERATED
        self.event_manager.state_changed.emit(self.state)

        if not open_pdf:
            return

        match sys.platform:
            case "win32":
                os.startfile(filename) # pyright: ignore
            case "linux" | "linux2":
                subprocess.run(["xdg-open", filename])
            case "darwin":
                subprocess.run(["open", filename])

    def pdf_options_changed(self, new_options: PdfOptions):
        self.pdf_options = new_options
