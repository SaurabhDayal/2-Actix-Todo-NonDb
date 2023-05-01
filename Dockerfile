# FROM rust
# WORKDIR /usr/src/actix-test
# COPY . .
# RUN cargo install --path .
# CMD ["todo"]
# EXPOSE 8080

FROM rust as builder
COPY . /app
WORKDIR /app
RUN cargo build --release


FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/todo /app/todo
WORKDIR /app
CMD ["./todo"]
EXPOSE 8080

