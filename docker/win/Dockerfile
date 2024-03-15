FROM rust:1.76

RUN rustup update
RUN rustup target add x86_64-pc-windows-msvc
RUN cargo install xwin
RUN xwin --accept-license splat --output /var/lib/xwin
RUN apt-get update && apt-get install lld -yy

WORKDIR /app

CMD ["cargo", "build", "--target=x86_64-pc-windows-msvc", "--release", "--bin", "tower"]
