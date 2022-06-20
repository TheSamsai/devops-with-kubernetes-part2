
FROM rust:latest AS builder

COPY . /app
WORKDIR /app/
RUN cargo build --release

from debian:latest
RUN mkdir -p /app
COPY --from=builder /app/target/release/todo-app /app
COPY --from=builder /app/templates /app/templates
WORKDIR /app/ 
RUN apt-get update -y && apt-get install wget -y
CMD ["./todo-app"]
