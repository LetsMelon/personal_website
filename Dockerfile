FROM alpine:3.17 as BLOG_BUILDER

RUN apk add rust cargo

RUN cargo new test_project && cd test_project && cargo add anyhow && cargo b && cd .. && rm -r /test_project

COPY /html-site-generator /html-site-generator

WORKDIR /html-site-generator

RUN mkdir dst && cargo r

FROM alpine:3.17 as MINIFY_BUILDER

RUN apk add minify

WORKDIR /files
COPY ./src ./src

RUN mkdir ./dst && minify -r -o dst/ src

FROM nginx:1.25-alpine

COPY ./nginx.conf /etc/nginx/conf.d/default.conf

COPY ./fonts /website/fonts
COPY ./assets /website/assets
COPY --from=MINIFY_BUILDER /files/dst/src /website/html
# COPY --from=BLOG_BUILDER /html-site-generator/dst /website/blog

EXPOSE 80
