This is a [Axum](https://docs.rs/axum/latest/axum/index.html) backend

## Local: Getting Started

```bash
cargo install cargo-watch
cargo watch -x run
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

## To-Do

Small fixes/features that don't really fit in in an issue

- [ ] Fix asset serving using tower's ServeDir/ServeFile methods
- [ ] Fix app_router function signature
- [ ] Add healthcheck api route
