from typing import Any, Literal, Optional
from PySide6.QtCore import Qt
from PySide6.QtGui import QFont
from PySide6.QtWidgets import QDialog, QGridLayout, QGroupBox, QHBoxLayout, QLabel, QPushButton, QRadioButton, QSpinBox, QStyle, QVBoxLayout, QWidget
from dataclasses import dataclass

@dataclass
class PdfOptions:
    grid_font_size: int = 16
    word_bank_font_size: int = 12
    page_size: Literal["letter", "a4"] | tuple[int, int] = "letter"
    margin: int = 36
    title: str = "Wordsearch"
    title_font_size: int = 24
    rows: int = 15
    cols: int = 15

class SettingPair(QWidget):
    other: QWidget

    def __init__(self, label: str, other: QWidget):
        super().__init__()

        main_layout = QHBoxLayout()
        main_layout.setSpacing(0)
        main_layout.setContentsMargins(0, 0, 0, 0)

        qlabel = QLabel(label)
        main_layout.addWidget(qlabel, alignment=Qt.AlignmentFlag.AlignLeft)

        main_layout.addSpacing(10)

        self.other = other
        main_layout.addWidget(self.other, alignment=Qt.AlignmentFlag.AlignRight)

        self.setLayout(main_layout)

    def get_value(self):
        return self.other.value() # pyright: ignore

class OptionsDialog(QDialog):
    margin: SettingPair

    letter_rb: QRadioButton
    a4_rb: QRadioButton

    page_width_sb: QSpinBox
    page_height_sb: QSpinBox

    page_margin_sb: QSpinBox

    grid_rows_sb: QSpinBox
    grid_cols_sb: QSpinBox

    grid_font_size: SettingPair
    word_bank_font_size: SettingPair
    title_font_size: SettingPair

    ok_button: QPushButton
    cancel_button: QPushButton

    def __init__(self, current_options: PdfOptions):
        super().__init__()

        self.setWindowTitle("PDF Options")
        # self.setGeometry(0, 0, 400, 400)

        bold_font = QFont()
        bold_font.setBold(True)

        main_layout = QVBoxLayout()

        grid_gb = QGroupBox("Grid")
        grid_bg_layout = QHBoxLayout()

        grid_bg_layout.addWidget(QLabel("Rows:"))
        self.grid_rows_sb = QSpinBox(minimum=0, maximum=1000, value=current_options.rows)
        grid_bg_layout.addWidget(self.grid_rows_sb)

        grid_bg_layout.addStretch()

        grid_bg_layout.addWidget(QLabel("Columns:"))
        self.grid_cols_sb = QSpinBox(minimum=0, maximum=1000, value=current_options.cols)
        grid_bg_layout.addWidget(self.grid_cols_sb)

        grid_gb.setLayout(grid_bg_layout)
        main_layout.addWidget(grid_gb)

        page_gb = QGroupBox("Page")
        page_gb_layout = QVBoxLayout()

        self.margin = SettingPair("Margin", QSpinBox(maximum=1000, value=current_options.margin))
        page_gb_layout.addWidget(self.margin)

        page_size_label = QLabel("Size")
        page_size_label.setFont(bold_font)
        page_gb_layout.addWidget(page_size_label)

        self.letter_rb = QRadioButton("Letter")
        page_gb_layout.addWidget(self.letter_rb)
        self.a4_rb = QRadioButton("A4")
        page_gb_layout.addWidget(self.a4_rb)
        other_rb = QRadioButton("Other:")
        page_gb_layout.addWidget(other_rb)
        if current_options.page_size == "letter":
            self.letter_rb.setChecked(True)
        elif current_options.page_size == "a4":
            self.a4_rb.setChecked(True)
        else:
            other_rb.setChecked(True)


        wh_layout = QHBoxLayout()

        wh_layout.addWidget(QLabel("Width (pts):"))
        self.page_width_sb = QSpinBox(minimum=0, maximum=1000, value=612)
        self.page_width_sb.setEnabled(False)
        wh_layout.addWidget(self.page_width_sb)

        wh_layout.addSpacing(10)

        wh_layout.addWidget(QLabel("Height (pts):"))
        self.page_height_sb = QSpinBox(minimum=0, maximum=1000, value=792)
        self.page_height_sb.setEnabled(False)
        wh_layout.addWidget(self.page_height_sb)

        page_gb_layout.addLayout(wh_layout)

        other_rb.toggled.connect(self._other_rb_toggled)

        page_gb.setLayout(page_gb_layout)
        main_layout.addWidget(page_gb)

        fonts_gb = QGroupBox("Font sizes (pts)")
        fonts_gb_layout = QVBoxLayout()

        self.title_font_size = SettingPair("Title", QSpinBox(maximum=100, value=current_options.title_font_size))
        fonts_gb_layout.addWidget(self.title_font_size)

        self.grid_font_size = SettingPair("Grid", QSpinBox(maximum=100, value=current_options.grid_font_size))
        fonts_gb_layout.addWidget(self.grid_font_size)

        self.word_bank_font_size = SettingPair("Word bank", QSpinBox(maximum=100, value=12))
        fonts_gb_layout.addWidget(self.word_bank_font_size)

        fonts_gb.setLayout(fonts_gb_layout)
        main_layout.addWidget(fonts_gb)

        _container = QHBoxLayout()

        self.cancel_button = QPushButton("Cancel")
        self.cancel_button.setIcon(self.style().standardIcon(QStyle.StandardPixmap.SP_DialogCancelButton))
        self.cancel_button.clicked.connect(self.reject)
        _container.addWidget(self.cancel_button)

        self.ok_button = QPushButton("Ok")
        self.ok_button.setIcon(self.style().standardIcon(QStyle.StandardPixmap.SP_DialogOkButton))
        self.ok_button.setDefault(True)
        self.ok_button.clicked.connect(self.accept)
        _container.addWidget(self.ok_button)

        main_layout.addLayout(_container)

        self.setLayout(main_layout)

    def pdf_options(self) -> PdfOptions:
        page_size = None
        if self.letter_rb.isChecked():
            page_size = "letter"
        elif self.a4_rb.isChecked():
            page_size = "a4"
        else:
            page_size = (self.page_width_sb.value(), self.page_height_sb.value())

        rows = self.grid_rows_sb.value()
        cols = self.grid_cols_sb.value()

        margin = self.margin.get_value()

        title_font_size = self.title_font_size.get_value()
        grid_font_size = self.grid_font_size.get_value()
        word_bank_font_size = self.word_bank_font_size.get_value()

        return PdfOptions(page_size=page_size, rows=rows, cols=cols, margin=margin,
                          title_font_size=title_font_size, grid_font_size=grid_font_size,
                          word_bank_font_size=word_bank_font_size)

    def _other_rb_toggled(self, checked):
        self.page_height_sb.setEnabled(checked)
        self.page_width_sb.setEnabled(checked)
