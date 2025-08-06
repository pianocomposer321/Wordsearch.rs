from PySide6.QtCore import QObject, Signal
from PySide6.QtGui import QFont, Qt
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

BUTTON_WIDTH = 180


class MainWindow(QMainWindow):
    app: QApplication
    event_manager: EventManager
    main_widget: QWidget
    main_font: QFont
    bold_font: QFont
    layout: QLayout
    word_list: QPlainTextEdit
    output_pdf_line_edit: QLineEdit
    close_button: QPushButton
    generate_button: QPushButton
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

        self.close_button = QPushButton("Close")
        self.close_button.setIcon(self.style().standardIcon(QStyle.StandardPixmap.SP_DialogCancelButton))
        self.close_button.setStatusTip("Quit application")
        self.close_button.setFixedWidth(200)
        self.close_button.clicked.connect(self.app.quit)
        _container.addWidget(self.close_button, alignment=Qt.AlignmentFlag.AlignLeft)

        self.generate_button = QPushButton("Generate")
        self.generate_button.setIcon(self.style().standardIcon(QStyle.StandardPixmap.SP_DialogApplyButton))
        self.generate_button.setStatusTip("Generate PDF using listed words")
        self.generate_button.setFixedWidth(200)
        self.generate_button.clicked.connect(self._generate_button_clicked)
        _container.addWidget(self.generate_button, alignment=Qt.AlignmentFlag.AlignRight)

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
            case AppState.GENERATED:
                self.status_message.setText("PDF generation successful.")
