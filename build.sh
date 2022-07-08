tag=$(cargo get version)
#cargo build --target aarch64-unknown-linux-musl --release
cargo sqlx prepare
docker build -t olepetersen/notifier:$tag .
docker push olepetersen/notifier:$tag