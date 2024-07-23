FROM node:22 as BUILDER
WORKDIR /website

COPY ["package.json", "package-lock.json", "./"]
RUN npm ci

COPY . .
RUN npm run build

FROM httpd:2.4 AS runtime
COPY --from=BUILDER /website/dist /usr/local/apache2/htdocs/
EXPOSE 80
