FROM node:22 as BUILDER

WORKDIR /website
COPY ["package.json", "package-lock.json", "./"]

RUN npm ci

COPY . .

# TODO: set '"prerender": false' in angular.json to 'true'
RUN npm run build -- --verbose

FROM node:22-alpine

WORKDIR /usr/app
COPY --from=BUILDER /website/dist/website ./

EXPOSE 4000
ENTRYPOINT [ "node", "server/server.mjs" ]
