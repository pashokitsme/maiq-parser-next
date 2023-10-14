FROM rustlang/rust:nightly-slim as builder

WORKDIR /src
COPY . /src

RUN \
  --mount=type=cache,id=s/089e3255-5585-4126-bced-b42eb9ceb953-/root/cargo,target=~/.cargo \
  ["cargo", "update"] \
  ["cargo", "fetch"]

RUN \
  --mount=type=cache,id=s/089e3255-5585-4126-bced-b42eb9ceb953-/root/cargo,target=~/.cargo \
  ["cargo", "build", "--release", "--package", "maiq-bot"]

FROM debian:bullseye-slim

WORKDIR /bin

ARG RUST_LOG=info
ARG RUST_LOG_STYLE=always

ARG SQLITE_PATH
ARG TELOXIDE_TOKEN

COPY --from=builder /src/target/release/maiq-bot maiq-bot

CMD [ "maiq-bot" ]