FROM node:16 as build

WORKDIR /app
COPY . /app

RUN yarn install && yarn build


FROM nginx:alpine

COPY --from=build /app/dist /usr/share/nginx/html