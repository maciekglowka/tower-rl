docker run --rm -v $(pwd):/app -v cargo_index:/usr/local/cargo -v $(pwd)/../rogalik:/rogalik -t tower_win
rm ./output/windows/ -rf
mkdir ./output/windows/
cp ./target/x86_64-pc-windows-msvc/release/tower.exe ./output/windows/monk_tower.exe