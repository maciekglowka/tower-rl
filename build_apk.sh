rm ./output/android/ -rf
mkdir ./output/android/
mkdir ./output/android/apk
docker run --rm -v $(pwd):/app -v cargo_index:/usr/local/cargo -v $(pwd)/../rogalik:/rogalik --env-file ./.apk_env -t tower_android

mv ./output/android/bundle.aab ./output/android/monk_tower.aab
cp ./target/release/apk/tower.apk ./output/android/apk/monk_tower.apk