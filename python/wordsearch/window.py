from PySide6.QtCore import QObject, Signal
from PySide6.QtGui import QFont
from PySide6.QtWidgets import (
    QApplication,
    QHBoxLayout,
    QLabel,
    QLayout,
    QLineEdit,
    QMainWindow,
    QPlainTextEdit,
    QPushButton,
    QStatusBar,
    QStyle,
    QVBoxLayout,
    QWidget,
)

from wordsearch.event_manager import AppState, EventManager


class MainWindow(QMainWindow):
    app: QApplication
    event_manager: EventManager
    main_widget: QWidget
    main_font: QFont
    bold_font: QFont
    layout: QLayout
    word_list: QPlainTextEdit
    output_pdf_line_edit: QLineEdit
    output_pdf_dialogue_button: QPushButton
    generate_button: QPushButton
    preview_button: QPushButton
    save_button: QPushButton
    status_bar: QStatusBar
    status_message: QLabel

    generate_signal = Signal()

    def __init__(self, app: QApplication, event_manager: EventManager):
        super().__init__()

        self.app = app
        self.main_font = QFont()
        self.main_font.setPointSize(12)
        self.app.setFont(self.main_font)

        self.event_manager = event_manager
        self.event_manager.state_changed.connect(self._app_state_changed)

        self.bold_font = QFont()
        self.bold_font.setBold(True)

        self.setWindowTitle("Wordsearch Generator")
        self.setGeometry(0, 0, 600, 400)

        self.layout = QVBoxLayout()

        word_list_label = QLabel("Word List:")
        word_list_label.setFont(self.bold_font)
        self.layout.addWidget(word_list_label)

        self.word_list = QPlainTextEdit()
        self.word_list.setFixedHeight(300)
        self.layout.addWidget(self.word_list)

        _container = QHBoxLayout()
        self.layout.addLayout(_container)

        self.save_button = QPushButton("Save as...")
        self.save_button.setIcon(self.style().standardIcon(QStyle.StandardPixmap.SP_DialogSaveButton))
        self.save_button.setStatusTip("Save PDF file")
        _container.addWidget(self.save_button)
        self.save_button.setEnabled(False)

        self.preview_button = QPushButton("Preview...")
        self.preview_button.setIcon(self.style().standardIcon(QStyle.StandardPixmap.SP_FileDialogContentsView))
        self.preview_button.setStatusTip("Preview PDF in default PDF viewer")
        _container.addWidget(self.preview_button)
        self.preview_button.setEnabled(False)

        self.generate_button = QPushButton("Generate")
        self.generate_button.setIcon(self.style().standardIcon(QStyle.StandardPixmap.SP_DialogApplyButton))
        self.generate_button.setStatusTip("Generate PDF using listed words")
        self.generate_button.clicked.connect(self._generate_button_clicked)
        _container.addWidget(self.generate_button)

        self.main_widget = QWidget()
        self.main_widget.setLayout(self.layout)
        self.setCentralWidget(self.main_widget)

        self.status_bar = QStatusBar(self)
        self.setStatusBar(self.status_bar)
        self.status_message = QLabel("Ready.")
        self.status_bar.addPermanentWidget(self.status_message)

    def get_words(self) -> str:
        return self.word_list.toPlainText()

    def _generate_button_clicked(self):
        self.generate_signal.emit()

    def _app_state_changed(self, new_state: AppState):
        match new_state:
            case AppState.READY:
                self.status_message.setText("Ready.")
            case AppState.GENERATING:
                self.status_message.setText("Generating...")
