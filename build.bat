@echo off
wasm-pack build --no-typescript -t web -d out --no-pack
xcopy /y index.html out