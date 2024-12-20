#!/bin/sh

echo "Make sure emsdk_env is activated"

cd client_src/config
./config-web-unix.sh
cd ../build/web
make
# cp *.html ../../../client/compiled/ # We don't need the html, because we have our own modified version
cp *.js ../../../client/compiled/
cp *.wasm ../../../client/compiled/