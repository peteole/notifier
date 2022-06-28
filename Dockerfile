FROM scratch
COPY target/release/notifier /notifier
ENTRYPOINT ["/notifier"]