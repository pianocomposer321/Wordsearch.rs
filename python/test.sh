#!/bin/sh

DIR=$(dirname $0)
DIR=$(realpath $DIR)

cd $DIR/../rust
cargo build
mkdir -p $DIR/bin
cp -r target/*/wordsearch $DIR/bin

cd $DIR
poetry run test
