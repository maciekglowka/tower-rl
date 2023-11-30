docker run --rm -v $(pwd):/app -v cargo_index:/usr/local/cargo -v $(pwd)/../rogalik:/rogalik --env-file ./.apk_env -t tower_android
rm ./output/android/ -rf
mkdir ./output/android/
cp ./target/release/apk/tower.apk ./output/android/monk_tower.apk