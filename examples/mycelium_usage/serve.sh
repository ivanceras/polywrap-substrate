#!/bin/sh
wasm-pack build --release --target web &&\

basic-http-server
