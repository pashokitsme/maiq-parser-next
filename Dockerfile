FROM rustlang/rust:nightly-slim as builder

WORKDIR /src
COPY . /src

RUN \
  --mount=type=cache,id=s/1bb9efb7-d8c5-4a31-9089-7fe535eaa480-/root/cargo,target=~/.cargo \
  ["cargo", "update"] \
  ["cargo", "fetch"]

RUN \
  --mount=type=cache,id=s/1bb9efb7-d8c5-4a31-9089-7fe535eaa480-/root/cargo,target=~/.cargo \
  ["cargo", "build", "--release"]

FROM debian:bullseye-slim

WORKDIR /bin

ARG RUST_LOG=info
ARG RUST_LOG_STYLE=always

ARG SQLITE_PATH
ARG TELOXIDE_TOKEN

COPY --from=builder /src/target/release/maiq-bot maiq-bot

CMD [ "maiq-bot" ]