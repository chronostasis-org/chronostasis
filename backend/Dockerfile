FROM rust:latest

WORKDIR /usr/src/backend
COPY . .

# Install cargo-watch for auto-reloading
RUN cargo install cargo-watch

# Install any system dependencies you need here

CMD ["cargo", "watch", "-x", "run"]