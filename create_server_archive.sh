#!/bin/bash

echo "Creating server archive..."
tar --exclude='server/node_modules' \
    --exclude='server/wasm/target' \
    --exclude='server/tests.js' \
    -czf server.tar.gz server/

echo "Archive created as server.tar.gz" 