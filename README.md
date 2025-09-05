## Project Structure

```
.
└── chronostasis/
    ├── frontend/
    │   └── ...
    └── backend/
        └── src/
            ├── api/
            │   ├── handlers/        # HTTP handlers (controllers)
            │   └── router.rs        # Route configuration (Axum router)
            ├── database/
            │   └── migrations/      # SeaORM migrations
            ├── common/              # Shared config, types, utils, errors
            │   └── cfg.rs
            ├── dto/                 # Data Transfer Objects (request/response structs)
            ├── entities/            # SeaORM entities
            ├── services/            # Business logic/services layer
            ├── models/              # Domain models (e.g., enums: UserRole, UserStatus...)
            └── static/              # Static files/assets (e.g., images)
```

## Run with docker compose

Start all services

```bash
docker compose up
```

## Prune docker's files

```bash
docker system prune -a --volumes
```

## To-Do

- [ ] enable docker's [file watch](https://docs.docker.com/compose/how-tos/file-watch/)
- [ ] add production and development states to Dockerfile
