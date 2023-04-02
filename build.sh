#!/usr/bin/bash
npm run build
cd backend
cargo build --release
