FROM rust:latest AS build

RUN rustup default stable && rustup update
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

COPY . .
RUN trunk build --release

FROM nginx:alpine

COPY --from=build /dist /usr/share/nginx/html
# need to add the application/wasm mime type to NGINX - it doesn't come with it by default!
RUN sed -i '97i application/wasm wasm;' /etc/nginx/mime.types