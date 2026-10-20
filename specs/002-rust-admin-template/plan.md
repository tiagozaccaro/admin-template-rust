# Implementation Plan: Rust-First Admin Template

**Branch**: `002-rust-admin-template` | **Date**: 2026-10-18 | **Spec**: spec.md

**Input**: Feature specification from `/specs/002-rust-admin-template/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command; its definition describes the execution workflow.

## Summary

Provide a reusable Rust-first full-stack admin template with Dioxus web and mobile, secure Google OIDC auth, multi-tenancy/roles, working dashboard, CRUD, AI chatbot with provider-independent abstraction and safe tool execution.

## Technical Context

**Language/Version**: Rust (stable; toolchain managed via rust-toolchain.toml; workspace with Cargo)

**Primary Dependencies**: Dioxus (web + mobile), Axum (API server), Tokio (async runtime), SQLx (PostgreSQL), serde/serde_json, jsonwebtoken (as needed for OIDC), openidconnect/oauth2 (OIDC), tracing (observability), tower (middleware), thiserror/anyhow, chrono, uuid

**Storage**: PostgreSQL via SQLx with versioned migrations (sqlx migrate)

**Testing**: cargo test (unit/integration), with platform-level tests for web/mobile where applicable

**Target Platform**: Web (modern browsers via Dioxus web), Mobile (Dioxus mobile targets - initial target(s) to be determined in Phase 0; Android/iOS considerations)

**Project Type**: Full-stack web + mobile application template (monorepo/workspace)

**Performance Goals**: Responsive UI, paginated lists, bounded API queries; AI streaming with low latency where supported; interactive requests with timeouts

**Constraints**: Rust-first; all authorization/tenant rules enforced server-side; secrets must never be in client bundles; no silent identity merging; explicit approval required for consequential AI actions; tenant isolation on all protected ops

**Scale/Scope**: Template for administrative apps - supports multi-tenant organizations, roles (Owner/Admin/Member/Viewer), CRUD, dashboard, AI chatbot with tools; scalable as modular monolith

**Open Decisions (from spec)**: 
- Dioxus version and mobile targets (Phase 0)
- Rust/UI evaluation outcome (go/no-go + fallback)
- OIDC library/session strategy
- Initial AI provider + capabilities
- Background jobs mechanism (if needed)
- Email/invitation delivery (if any)
- Deployment target and secret management
- Final crate boundaries (post-Phase 0)


## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**I. Rust-First Technology**
- [x] Rust is primary for logic/API/AI; Dioxus for web+mobile
- [x] Axum + Tokio planned; PostgreSQL via SQLx with migrations
- [x] Non-Rust tooling scoped appropriately

**II. Web and Mobile Feature Parity**
- [x] Parity required for supported capabilities; platform-specific presentation allowed
- [x] Shared contracts/validation/auth rules; parity matrix to be maintained

**III. Reusable and Modular Architecture**
- [x] Clear boundaries planned; modular monolith preferred; microservices only with justification
- [x] UI components reusable and not embedding app-specific logic

**IV. Design System and Rust/UI Evaluation**
- [x] Rust/UI to be evaluated in Phase 0 with cross-platform POC and documented go/no-go

**V. Authentication and Identity**
- [x] Google OIDC first; extensible provider abstraction; no silent identity merging; server-side secrets

**VI. Security, Privacy, and Authorization**
- [x] Server-side enforcement of auth/authorization/tenant boundaries; CSRF considerations for cookies; explicit approval for consequential AI actions; audit logging

**VII. Multi-Tenancy and Roles**
- [x] Multi-tenant with memberships; Owner/Admin/Member/Viewer roles; tenant isolation verified

**VIII. AI Provider Independence and Safe Execution**
- [x] Provider-independent interfaces; typed tools with validation/auth/approval/audit; tenant context enforced

**IX. Testing and Quality**
- [x] Unit/integration/platform tests planned; CI gates for fmt/lint/build/test

**X. Accessibility and User Experience**
- [x] Focus states, labels, validation feedback, practical touch targets planned

**XI. Observability and Operations**
- [x] Health/readiness, structured logs, tracing, request IDs, bounded timeouts

**XII. Documentation and Genericity**
- [x] Template must remain generic; extension guide required

**XIII. Spec-Driven Development**
- [x] Following Spec Kit workflow as installed

**XIV. Pull Requests and Release Governance**
- [x] Review gates, CI must pass, no credentials

**Initial Assessment**: No unjustified violations. All gates align with constitution.


## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
apps/
├── web/                  # Dioxus web application
├── mobile/               # Dioxus mobile application
├── api/                  # Axum API server
└── worker/               # Background jobs (introduced only if needed)

crates/
├── ui/                   # Shared design primitives
├── ui-admin/             # Admin-specific UI patterns
├── ui-ai/                # Reusable AI interface components
├── app-core/             # Shared application logic
├── contracts/            # Request/response DTOs (shared)
├── api-client/            # Typed API client (shared by web/mobile)
├── auth/                 # Authentication abstractions
├── permissions/         # Authorization and policy logic
├── tenancy/              # Tenant and membership logic
├── database/             # SQLx persistence, migrations helpers
├── ai-core/              # Provider-independent AI interfaces
├── ai-providers/         # Provider adapters
├── ai-agents/            # Tool definitions and execution policies
├── config/               # Validated configuration
└── observability/        # Shared tracing/telemetry

migrations/               # SQLx migrations
infrastructure/           # Deployment/configuration (docker, etc.)
tests/                    # Cross-cutting integration/E2E tests
docs/                     # Documentation
specs/                    # Spec Kit artifacts
Cargo.toml                # Workspace root
rust-toolchain.toml
README.md
```

**Structure Decision**: Cargo workspace with `apps/*` for runnable binaries (web, mobile, api, optional worker) and `crates/*` for shared libraries. This matches the proposed layout in the spec while allowing crate boundaries to be refined after Phase 0 based on actual needs.


## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
