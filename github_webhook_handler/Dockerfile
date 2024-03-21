FROM rustlang/rust:nightly-slim as BUILDER

WORKDIR /html-site-generator

COPY . .

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,sharing=private,target=/html-site-generator/target \
    cargo build --release --package github_webhook_handler && \
    cp target/release/github_webhook_handler /bin/github_webhook_handler

FROM debian:bookworm-slim

EXPOSE 3000

COPY --from=BUILDER /bin/github_webhook_handler /bin/github_webhook_handler

COPY . /tmp/melcher_io

ENTRYPOINT [ "/bin/github_webhook_handler" ]
