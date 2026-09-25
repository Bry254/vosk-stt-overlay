#!/bin/bash
# Archivo de compilacion y descarga de dependencias
wget https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip && unzip vosk-linux-x86_64-0.3.45.zip && cp ./vosk-linux-x86_64-0.3.45/libvosk.so ./lib
cargo build --release
cp ./lib/libvosk.so ./target/release/;
cargo run --release
