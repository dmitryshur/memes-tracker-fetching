FROM rust

WORKDIR /app
ADD . /app

EXPOSE 3000

ENV ADDRESS 127.0.0.1:3000

ENTRYPOINT ["/app/target/debug/memes-tracker-fetching"]
