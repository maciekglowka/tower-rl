rm ./output/linux/ -rf
mkdir ./output/linux/
cargo build --release --bin tower
cp ./target/release/tower ./output/linux/monk_tower
