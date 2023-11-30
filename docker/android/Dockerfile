FROM rust:1.74

RUN apt-get update && apt-get install android-sdk -yy
RUN mkdir /usr/lib/android-sdk/cmdline-tools/
WORKDIR /usr/lib/android-sdk/cmdline-tools
RUN wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip
RUN unzip commandlinetools-linux-9477386_latest.zip
RUN mv /usr/lib/android-sdk/cmdline-tools/cmdline-tools/ /usr/lib/android-sdk/cmdline-tools/latest/
RUN yes | /usr/lib/android-sdk/cmdline-tools/latest/bin/sdkmanager --licenses
RUN /usr/lib/android-sdk/cmdline-tools/latest/bin/sdkmanager --install "ndk;25.2.9519653" --channel=0
RUN /usr/lib/android-sdk/cmdline-tools/latest/bin/sdkmanager "platform-tools" "platforms;android-31"

RUN rustup update
RUN rustup target add aarch64-linux-android
RUN rustup target add armv7-linux-androideabi
RUN rustup target add i686-linux-android
RUN rustup target add x86_64-linux-android
RUN cargo install cargo-apk

ENV ANDROID_HOME=/usr/lib/android-sdk
ENV ANDROID_NDK_ROOT=/usr/lib/android-sdk/ndk/25.2.9519653

WORKDIR /app/crates/hike

CMD ["cargo", "apk", "build", "--release"]
