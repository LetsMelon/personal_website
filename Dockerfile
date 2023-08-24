FROM alpine:3.17 as BLOG_BUILDER

RUN apk add rust cargo minify

RUN cargo new test_project && cd test_project && cargo add anyhow && cargo b && cd .. && rm -r /test_project

WORKDIR /html-site-generator

COPY . .

RUN mkdir ./dst && cargo b && ./target/debug/melcher_io > ./dest/index.html

FROM nginx:1.25-alpine

COPY ./nginx.conf /etc/nginx/conf.d/default.conf

COPY ./fonts /website/fonts
COPY ./assets /website/assets
COPY --from=BLOG_BUILDER /html-site-generator/dst /website/html

EXPOSE 80
