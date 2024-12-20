#!/bin/sh

echo "Make sure emsdk_env is activated"

cd client_src/config
./config-web-unix.sh
cd ../build/web
make
cp *.html ../../../client/
cp *.js ../../../client/
cp *.wasm ../../../client/