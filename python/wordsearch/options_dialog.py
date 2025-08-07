from typing import Optional
from PySide6.QtCore import Qt
from PySide6.QtWidgets import QDialog, QGridLayout, QGroupBox, QHBoxLayout, QLabel, QRadioButton, QSpinBox, QVBoxLayout, QWidget

class SettingPair(QWidget):
    spinbox: QSpinBox

    def __init__(self, label: str, default_value: int = 0):
        super().__init__()

        main_layout = QHBoxLayout()

        qlabel = QLabel(label)
        main_layout.addWidget(qlabel, alignment=Qt.AlignmentFlag.AlignLeft)

        self.spinbox = QSpinBox(value=default_value)
        main_layout.addWidget(self.spinbox, alignment=Qt.AlignmentFlag.AlignRight)

        self.setLayout(main_layout)

    def get_value(self):
        return self.spinbox.value()

class OptionsDialog(QDialog):
    page_gb: QGroupBox
    grid_gb: QGroupBox

    page_width_sb: QSpinBox
    page_height_sb: QSpinBox

    grid_rows_sb: QSpinBox
    grid_cols_sb: QSpinBox

    def __init__(self, parent: Optional[QWidget] = None):
        super().__init__()

        self.setWindowTitle("PDF Options")
        # self.setGeometry(0, 0, 400, 400)

        main_layout = QVBoxLayout()

        self.page_gb = QGroupBox("Page")
        page_gb_layout = QVBoxLayout()

        letter_rb = QRadioButton("Letter")
        letter_rb.setChecked(True)
        page_gb_layout.addWidget(letter_rb)
        a4_rb = QRadioButton("A4")
        page_gb_layout.addWidget(a4_rb)
        other_rb = QRadioButton("Other:")
        page_gb_layout.addWidget(other_rb)

        wh_container = QWidget()
        wh_container.setEnabled(False)
        wh_layout = QHBoxLayout()

        wh_layout.addWidget(QLabel("Width (pts):"))
        self.page_width_sb = QSpinBox(minimum=0, maximum=1000, value=612)
        wh_layout.addWidget(self.page_width_sb)

        wh_layout.addSpacing(10)

        wh_layout.addWidget(QLabel("Height (pts):"))
        self.page_height_sb = QSpinBox(minimum=0, maximum=1000, value=792)
        wh_layout.addWidget(self.page_height_sb)

        wh_container.setLayout(wh_layout)
        page_gb_layout.addWidget(wh_container)

        other_rb.toggled.connect(lambda checked: wh_container.setEnabled(checked))

        self.page_gb.setLayout(page_gb_layout)
        main_layout.addWidget(self.page_gb)

        self.grid_gb = QGroupBox("Grid")
        grid_bg_layout = QHBoxLayout()

        grid_bg_layout.addWidget(QLabel("Rows:"))
        self.grid_rows_sb = QSpinBox(minimum=0, maximum=1000, value=15)
        grid_bg_layout.addWidget(self.grid_rows_sb)

        grid_bg_layout.addStretch()

        grid_bg_layout.addWidget(QLabel("Columns:"))
        self.grid_cols_sb = QSpinBox(minimum=0, maximum=1000, value=15)
        grid_bg_layout.addWidget(self.grid_cols_sb)

        self.grid_gb.setLayout(grid_bg_layout)
        main_layout.addWidget(self.grid_gb)

        self.setLayout(main_layout)
