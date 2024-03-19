# syntax=docker/dockerfile:1.2
FROM rustlang/rust:nightly-slim as BLOG_BUILDER

WORKDIR /html-site-generator

COPY . .

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,sharing=private,target=/html-site-generator/target \
    cargo build --release && \
    cp target/release/melcher_io /bin/melcher_io

RUN mkdir ./dst && /bin/melcher_io build ./dst

FROM nginx:1.25-alpine

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY ./fonts /website/fonts
COPY ./assets /website/assets
COPY ./static /website/html

COPY --from=BLOG_BUILDER /html-site-generator/dst /website/html

EXPOSE 80
