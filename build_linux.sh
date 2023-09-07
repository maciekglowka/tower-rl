cargo build --release --bin tower
rm ./output/linux -rf
mkdir ./output/linux
cp ./assets ./output/linux -r
cp ./target/release/tower ./output/linux
cd ./output/linux && zip -r tower.zip *