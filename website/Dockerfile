FROM node:22 as BUILDER
WORKDIR /website

COPY ["package.json", "package-lock.json", "./"]
RUN npm ci

COPY . .
RUN npm run build

# nginx with brotli
FROM fholzer/nginx-brotli:v1.23.4 

COPY ./nginx.conf /etc/nginx/nginx.conf

COPY --from=BUILDER /website/dist /usr/share/nginx/html

EXPOSE 80
