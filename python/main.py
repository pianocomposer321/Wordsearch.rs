import sys
from wordsearch import App

def main():
    app = App(sys.argv)
    sys.exit(app.exec())

if __name__ == '__main__':
    main()
