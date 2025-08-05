import sys
from .app import App

def main():
    app = App(sys.argv)
    sys.exit(app.exec())
