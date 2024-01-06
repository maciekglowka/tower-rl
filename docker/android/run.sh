cargo apk build --release

set -e

TEMP=$(mktemp -d)

/usr/lib/extra/aapt2/aapt2 convert ../../target/release/apk/tower.apk --output-format proto -o $TEMP/app_proto.apk

cd $TEMP

unzip app_proto.apk
mkdir manifest

mv AndroidManifest.xml manifest/

rm app_proto.apk
rm META-INF -rf

zip -r base.zip *

cd -

java -jar /usr/lib/extra/bundletool-all-1.15.6.jar build-bundle --modules=$TEMP/base.zip --output=../../output/android/bundle.aab

jarsigner -keystore $CARGO_APK_RELEASE_KEYSTORE -storepass $CARGO_APK_RELEASE_KEYSTORE_PASSWORD ../../output/android/bundle.aab tower