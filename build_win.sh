docker run --rm -v $(pwd):/app -v cargo_index:/usr/local/cargo -v $(pwd)/../rogalik:/rogalik -t tower_win
cp ./target/x86_64-pc-windows-msvc/release/tower.exe ./output/monk_tower.exe