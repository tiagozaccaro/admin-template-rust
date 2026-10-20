# Quickstart: Rust-First Admin Template

**Date**: 2026-10-18  
**Feature**: 002-rust-admin-template

## Prerequisites

- Rust stable (see rust-toolchain.toml)
- Cargo + cargo-workspace
- PostgreSQL 14+ (local dev)
- Dioxus CLI: `cargo install dioxus-cli`
- For mobile (Android): Android SDK, NDK, Java; (iOS): Xcode on macOS
- Google OAuth credentials (client ID/secret) for dev

## Local Development Setup

1. Clone repo and navigate to project root.
2. Copy `.env.example` to `.env` and configure:
   - DATABASE_URL=postgres://admin:admin@localhost:5434/admin_template_dev
   - GOOGLE_CLIENT_ID=...
   - GOOGLE_CLIENT_SECRET=...
   - API_BASE_URL=http://localhost:8080
   - WEB_BASE_URL=http://localhost:3000
   - JWT/SESSION_SECRET=dev-secret-change-me
   - AI_PROVIDER=openai-compatible (or configured)
   - AI_BASE_URL=...
   - AI_API_KEY=...
3. Start PostgreSQL (docker compose or local).
4. Run migrations: `cargo sqlx migrate run` (or via sqlx-cli).
5. Build workspace: `cargo build --workspace`.

## Running Services

- API: `cargo run -p api` (default port 8080)
- Web (Dioxus): `dx serve --package web` (port 3000)
- Mobile (dev): `dx serve --package mobile` or `cargo mobile run` depending on target

## Validation Scenarios

### Scenario 1: Auth flow
1. Start API + web.
2. Navigate to /login → redirected to Google OAuth.
3. Complete OAuth → redirected back, session created.
4. Access protected route (/dashboard) - succeeds.
5. Logout - session revoked.

### Scenario 2: Multi-tenancy & roles
1. Create tenant as authenticated user (becomes owner).
2. Invite member with role 'member' via API/UI.
3. Accept invitation → membership created.
4. Switch tenants; verify data isolation.
5. As viewer, attempt destructive action → denied (403). Server-side enforcement verified.

### Scenario 3: Dashboard with real data
1. With tenant context, load /dashboard.
2. Verify metrics/activity load from API (real data). Check loading/empty/error states.

### Scenario 4: CRUD + validation
1. List members (search/filter/paginate).
2. Create/update member with invalid input → validation errors shown (field + server).
3. Delete with confirmation → authorized deletion succeeds.

### Scenario 5: AI chatbot
1. Configure AI provider in env.
2. Open chatbot (web/mobile). Send message.
3. Receive response (streaming if supported). Conversation persisted and isolated.
4. Simulate provider unavailable → clear error shown without crash.

### Scenario 6: AI tools with approval
1. Trigger tool that requires approval.
2. Execution returns pending; not executed until approved.
3. Approve as authorized user → executes and audited. Deny → not executed with reason.

### Scenario 7: Parity check
1. Exercise core flows on web and mobile. Verify same supported operations, auth rules, validation semantics.
2. Confirm feature-parity matrix entries for implemented features.

## Testing

- Unit: `cargo test --workspace --lib`
- Integration: `cargo test --workspace --test '*'`
- Lint/format: `cargo fmt --check && cargo clippy --all-targets --all-features`

## Deployment Notes

- Build container images for api and web assets. 
- Configure production secrets via platform secret store (never in repo). 
- Run migrations on deploy. 
- Expose health/readiness endpoints for orchestrators.
