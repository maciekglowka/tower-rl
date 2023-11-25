cargo build --target wasm32-unknown-unknown --release
rm ./output/wasm/ -rf
mkdir ./output/wasm/
wasm-bindgen --out-dir ./output/wasm/ --target web ./target/wasm32-unknown-unknown/release/tower.wasm
cp ./wasm-assets/index.html ./output/wasm/ 
# cd wasm-out && zip -r wasm.zip *
