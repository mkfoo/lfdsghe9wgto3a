#!/bin/bash

set -e

TARGET="wasm32-unknown-unknown"

if [[ "$1" == debug ]] ; then
    cargo build "--target=$TARGET"
    cd "target/$TARGET/debug"
    cp eggine.wasm main.wasm
    webfsd -i 127.0.0.1 -p 5000 -f index.html 2> /dev/null
elif [[ "$1" == release ]] ; then
    cargo build --release "--target=$TARGET"
    cd "target/$TARGET/release"
    wasm-opt -Oz --strip-debug --dce -o main.wasm eggine.wasm
    webfsd -i 127.0.0.1 -p 5001 -f index.html 2> /dev/null
else
    echo "no target specified"
    exit 1
fi


