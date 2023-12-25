rm ./output/android/ -rf
mkdir ./output/android/
docker run --rm -v $(pwd):/app -v cargo_index:/usr/local/cargo -v $(pwd)/../rogalik:/rogalik --env-file ./.apk_env -t tower_android

mv ./output/android/bundle.aab ./output/android/monk_tower.aab