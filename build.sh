tag="0.0.4"
#cargo build --target aarch64-unknown-linux-musl --release
docker build --no-cache -t olepetersen/notifier:$tag .
docker push olepetersen/notifier:$tag