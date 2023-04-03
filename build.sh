#!/usr/bin/bash
mkdir -p bin/{server,page}

npm run build

cp -rf dist/* bin/page

cd backend || exit
cargo build --release

cp -f target/release/backend ../bin/server
