#!/bin/bash
set -e
docker run --rm -v $(pwd):/app -v cargo_index:/usr/local/cargo -v $(pwd)/../rogalik:/rogalik -t tower_win cargo build --target=x86_64-pc-windows-msvc
target/x86_64-pc-windows-msvc/debug/tower.exe
