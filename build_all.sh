rm ./output -rf
mkdir ./output
./build_linux.sh
./build_wasm.sh
./build_win.sh