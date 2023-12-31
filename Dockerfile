FROM rust:slim-bullseye as builder

WORKDIR /src
COPY . .

ENV SQLX_OFFLINE 1

RUN \
  --mount=type=cache,id=s/089e3255-5585-4126-bced-b42eb9ceb953-/root/cargo,target=~/.cargo \
  ["cargo", "fetch"] \
  ["cargo", "update"]

RUN \
  --mount=type=cache,id=s/089e3255-5585-4126-bced-b42eb9ceb953-/root/cargo,target=~/.cargo \
  ["cargo", "build", "--release", "--package", "maiq-bot"]

FROM debian:bullseye-slim

WORKDIR /bin

ARG RUST_LOG=info
ARG RUST_LOG_STYLE=always

ARG SQLITE_PATH
ARG TELOXIDE_TOKEN
ARG RAILWAY_GIT_BRANCH
ARG RAILWAY_GIT_COMMIT_SHA

COPY --from=builder /src/target/release/maiq-bot maiq-bot

CMD [ "maiq-bot" ]