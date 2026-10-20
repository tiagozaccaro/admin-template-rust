# Deployment Reference Architecture

## Components
- API server (Axum/Tokio)
- Web app (Dioxus web build, served statically)
- PostgreSQL database
- Optional worker for background jobs

## Containerization
- api/Dockerfile - API service
- web/Dockerfile - Static web assets
- docker-compose.prod.yml - Orchestration

## Configuration
- All secrets via environment variables
- Never commit credentials
- Health/readiness endpoints: /healthz, /readyz
- Database migrations run on deploy

## Deployment Checklist
- [ ] Set all required env vars from .env.example
- [ ] Run migrations: cargo sqlx migrate run
- [ ] Build and deploy containers
- [ ] Verify health endpoints
- [ ] Configure TLS and CORS appropriately
