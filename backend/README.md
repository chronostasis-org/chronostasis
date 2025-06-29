This is a [Axum](https://docs.rs/axum/latest/axum/index.html) backend

## Local: Getting Started

Install `cargo-watch`

```bash
cargo install cargo-watch
```

First terminal (recompile after changing src folder):

```bash
cargo watch -q -c -w src/ -x run
```

Second terminal (tests):

```bash
cargo watch -q -c -w tests/ -x "test -q oneforall -- --nocapture"
```

## Docker: Getting Started

Build the image

```bash
docker build -t cs-backend .
```

Run container

```bash
docker run --rm -it -p 8000:8000 -v "$PWD":/usr/src/backend cs-backend
```
