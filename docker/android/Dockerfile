FROM rust:1.74

RUN apt-get update && apt-get install android-sdk -yy
RUN mkdir /usr/lib/android-sdk/cmdline-tools/
WORKDIR /usr/lib/android-sdk/cmdline-tools
RUN wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip
RUN unzip commandlinetools-linux-9477386_latest.zip
RUN mv /usr/lib/android-sdk/cmdline-tools/cmdline-tools/ /usr/lib/android-sdk/cmdline-tools/latest/
RUN yes | /usr/lib/android-sdk/cmdline-tools/latest/bin/sdkmanager --licenses
RUN /usr/lib/android-sdk/cmdline-tools/latest/bin/sdkmanager --install "ndk;25.2.9519653" --channel=0
RUN /usr/lib/android-sdk/cmdline-tools/latest/bin/sdkmanager "platform-tools" "platforms;android-33"

RUN rustup update
RUN rustup target add aarch64-linux-android
RUN rustup target add armv7-linux-androideabi
RUN rustup target add i686-linux-android
RUN rustup target add x86_64-linux-android
RUN cargo install cargo-apk

RUN apt-get update && apt-get install zip

RUN mkdir /usr/lib/extra/
WORKDIR /usr/lib/extra/
RUN wget https://github.com/google/bundletool/releases/download/1.15.6/bundletool-all-1.15.6.jar
RUN wget https://dl.google.com/dl/android/maven2/com/android/tools/build/aapt2/8.2.0-10154469/aapt2-8.2.0-10154469-linux.jar
RUN chmod +x bundletool-all-1.15.6.jar
RUN unzip aapt2-8.2.0-10154469-linux.jar -d aapt2

COPY run.sh /usr/lib/run.sh
RUN chmod +x /usr/lib/run.sh

ENV ANDROID_HOME=/usr/lib/android-sdk
ENV ANDROID_NDK_ROOT=/usr/lib/android-sdk/ndk/25.2.9519653

WORKDIR /app/crates/hike

CMD /usr/lib/run.sh
