set -e

rm android_assets/native_activity/app/src/main/jniLibs/* -r
cargo ndk -p 35 \
  -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 \
  -o android_assets/native_activity/app/src/main/jniLibs \
  build --release

cd android_assets/native_activity

mv app/src/main/jniLibs/arm64-v8a/libtower.so app/src/main/jniLibs/arm64-v8a/libmain.so
mv app/src/main/jniLibs/armeabi-v7a/libtower.so app/src/main/jniLibs/armeabi-v7a/libmain.so
mv app/src/main/jniLibs/x86/libtower.so app/src/main/jniLibs/x86/libmain.so
mv app/src/main/jniLibs/x86_64/libtower.so app/src/main/jniLibs/x86_64/libmain.so

rm app/build/outputs/apk/* -rf
./gradlew build


TEMP=$(mktemp -d)

/usr/lib/extra/aapt2/aapt2 convert app/build/outputs/apk/release/app-release-unsigned.apk --output-format proto -o $TEMP/app_proto.apk

cd $TEMP

unzip app_proto.apk
mkdir manifest

mv AndroidManifest.xml manifest/

rm app_proto.apk
rm META-INF -rf
rm DebugProbesKt.bin 
rm kotlin/ -rf

zip -r base.zip *

cd -

java -jar /usr/lib/extra/bundletool-all-1.15.6.jar build-bundle --modules=$TEMP/base.zip --output=bundle.aab
jarsigner -keystore $CARGO_APK_RELEASE_KEYSTORE -storepass $CARGO_APK_RELEASE_KEYSTORE_PASSWORD bundle.aab tower
