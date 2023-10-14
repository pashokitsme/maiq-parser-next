FROM rustlang/rust:nightly-slim as builder

WORKDIR /src
COPY . /src

ARG RAILWAY_SERVICE_ID=cache

RUN \
  --mount=type=cache,id=${RAILWAY_SERVICE_ID}/.cargo,target=~/.cargo \
  ["cargo", "update"] \
  ["cargo", "fetch"]

RUN \
  --mount=type=cache,id=${RAILWAY_SERVICE_ID}/.cargo,target=~/.cargo \
  ["cargo", "build", "--release"]

FROM debian:bullseye-slim

WORKDIR /bin

ARG RUST_LOG=info
ARG RUST_LOG_STYLE=always

ARG SQLITE_PATH
ARG TELOXIDE_TOKEN

COPY --from=builder /src/target/release/maiq-bot maiq-bot

CMD [ "maiq-bot" ]