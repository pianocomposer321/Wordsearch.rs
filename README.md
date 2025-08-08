# Wordsearch.rs

A simple worsearch generator written in rust, with a frontend GUI in python.

## Installation & Usage

#### Using the release (recommended):

For the CLI, download wordsearch-linux or wordsearch.exe (for linux or windows
respectively). This is a standalone executable binary, and it should just work
as is.

For the python UI, download wordsearch_ui-linux.tar.gz or
wordsearch_ui-windows.zip. Extract the archive and run the wordsearch_ui
executable. Alternatively, you can download the source code archive
(wordsearch_ui-0.1.tar.gz) and run the code using a python interpreter `python3
main.py`.

#### Cloning the repo:

```console
$ git clone https://github.com/pianocomposer321/Wordsearch.rs
$ cd python
$ python3 main.py # run the python frontend ui
$ cd ../rust
$ cargo run -- words.txt -o output.pdf # run the rust CLI
$ cargo install --path . # install the rust binary
```

## Demo

[2025-08-08 13-01-53.webm](https://github.com/user-attachments/assets/75ed52f5-70d2-48fe-8418-cc41f8c9fce3)
