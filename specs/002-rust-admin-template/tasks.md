# Tasks: Rust-First Admin Template

**Input**: Design documents from `/specs/002-rust-admin-template/`  
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/api-contracts.md, quickstart.md  

**Organization**: Tasks are grouped by user story (P0 first, then P1) to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (US1..US12) - only for user story phases
- Include exact file paths in descriptions

## Path Conventions

Based on implementation plan: Cargo workspace with apps/* (web, mobile, api, worker) and crates/* (shared libs), migrations/, infrastructure/, tests/, docs/, specs/.

---

## Phase 1: Setup (Project Initialization)

**Purpose**: Initialize workspace, tooling, and basic structure

- [x] T001 Create Cargo workspace structure per plan (Cargo.toml, rust-toolchain.toml, apps/, crates/, migrations/, infrastructure/, tests/, docs/)
- [x] T002 Initialize workspace Cargo.toml with members and shared metadata
- [x] T003 [P] Create rust-toolchain.toml (Rust stable as determined)
- [x] T004 [P] Setup .env.example with all required config vars (DB, Google OAuth, AI provider, session)
- [x] T005 [P] Setup Docker compose for PostgreSQL (infrastructure/docker-compose.dev.yml)
- [x] T006 [P] Configure formatting/linting (rustfmt.toml, clippy config if needed)
- [x] T007 [P] Setup basic CI workflow (.github/workflows/ci.yml)
- [x] T008 [P] Initialize README.md with setup instructions

**Checkpoint**: Workspace skeleton ready.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST complete before user stories. All stories depend on this.

**CRITICAL - No user story work until complete**

- [x] T009 Setup config crate (crates/config) with environment validation (figment/env)
- [x] T010 [P] Setup observability crate (crates/observability) with tracing, logging, request IDs
- [x] T011 [P] Setup database crate (crates/database) with SQLx connection pool, migration helpers
- [x] T012 [P] Setup contracts crate (crates/contracts) with shared DTOs/types (serde)
- [x] T013 [P] Setup api-client crate (crates/api-client) for typed API calls (used by web/mobile)
- [x] T014 [P] Setup auth crate (crates/auth) with auth abstractions, session models
- [x] T015 [P] Setup permissions crate (crates/permissions) with RBAC (Owner/Admin/Member/Viewer) and permission checks
- [x] T016 [P] Setup tenancy crate (crates/tenancy) with tenant/membership/invitation domain logic
- [x] T017 [P] Setup app-core crate (crates/app-core) with shared domain logic and validation rules
- [x] T018 [P] Setup migrations (migrations/) with initial schema for all entities from data-model.md
- [x] T019 Setup API binary (apps/api) with Axum, middleware (auth, tenant, request-id), health endpoints
- [x] T020 [P] Setup UI crates - ui, ui-admin, ui-ai (Dioxus components, design tokens, themes)
- [x] T021 [P] Setup AI core crates (crates/ai-core, ai-providers, ai-agents) with provider traits and tool interfaces
- [x] T022 Setup web app (apps/web) with Dioxus web, routing, auth integration
- [x] T023 Setup mobile app (apps/mobile) with Dioxus mobile, platform adapters
- [x] T024 [P] Create shared test utilities (tests/common/)

**Checkpoint**: Foundation complete. All user stories can begin.

---

## Phase 3: User Story 1 - Set up template for new project (Priority: P0) 🎯 MVP

**Goal**: Documented setup and repeatable build/test commands so developers can start without reverse-engineering.

**Independent Test**: Developer from clean checkout follows README to build, run migrations, start API/web/mobile dev targets.

### Implementation for US1

- [x] T025 [P] [US1] Finalize README.md with complete setup instructions, env vars, commands (migrations, build, run)
- [x] T026 [P] [US1] Add quickstart validation script or documented commands matching quickstart.md
- [x] T027 [US1] Verify workspace builds (`cargo build --workspace`)
- [x] T028 [US1] Verify tests run (`cargo test --workspace --lib`)
- [x] T029 [US1] Document Dioxus/mobile prerequisites and commands

**Checkpoint**: US1 complete and independently testable.

---

## Phase 4: User Story 2 - Sign in with Google on web and mobile (Priority: P0)

**Goal**: Secure Google OIDC login/logout with session management on both platforms.

**Independent Test**: Can sign in with Google on web and mobile, logout revokes session, invalid/expired sessions handled.

### Implementation for US2

- [x] T030 [P] [US2] Implement OIDC client with openidconnect (auth crate)
- [x] T031 [P] [US2] Implement session store in DB (database/models + tenancy/auth integration)
- [x] T032 [P] [US2] API endpoints: login start, callback, logout, me (apps/api/src/routes/auth.rs)
- [x] T033 [US2] Session middleware for protected routes
- [x] T034 [US2] Web auth flow (apps/web) - login UI, callback handling, session persistence
- [x] T035 [US2] Mobile auth flow (apps/mobile) - platform-appropriate auth with secure storage
- [x] T036 [US2] Account linking logic (no silent merge; key by provider+subject)
- [x] T037 [US2] Auth integration tests (unit/integration)

**Checkpoint**: US2 complete.

---

## Phase 5: User Story 3 - Consistent product across platforms (Priority: P0)

**Goal**: Same supported capabilities, data, business rules, auth semantics across web/mobile.

**Independent Test**: Feature parity matrix validates equivalent behavior.

### Implementation for US3

- [x] T038 [P] [US3] Create feature parity matrix (docs/parity-matrix.md) with all supported features
- [x] T039 [P] [US3] Define shared API client usage in both apps (crates/api-client)
- [x] T040 [P] [US3] Share contracts/validation rules (contracts crate)
- [x] T041 [US3] Platform-specific adapters only where needed (navigation, storage, OAuth)
- [x] T042 [US3] Add parity tests/validation

**Checkpoint**: US3 foundations in place.

---

## Phase 6: User Story 4 - Manage tenants and memberships (Priority: P0)

**Goal**: Multi-tenancy with tenant switching, memberships, invitations with configurable delivery.

**Independent Test**: User in multiple tenants switches and sees only tenant-scoped data; cross-tenant access denied.

### Implementation for US4

- [x] T043 [P] [US4] Tenant/membership domain logic (crates/tenancy)
- [x] T044 [P] [US4] Invitation token generation and validation
- [x] T045 [P] [US4] Email sender trait with logging-only impl (configurable)
- [x] T046 [P] [US4] API endpoints: tenants CRUD, switch, members, invitations (accept/reject/revoke)
- [x] T047 [US4] Web UI: tenant selector, members list, invite form
- [x] T048 [US4] Mobile UI: tenant selector, members list, invite form
- [x] T049 [US4] Tenant middleware/scoping on all protected ops
- [x] T050 [US4] Tests for tenant isolation and invitation flows

**Checkpoint**: US4 complete.

---

## Phase 7: User Story 5 - Enforce roles and permissions (Priority: P0)

**Goal**: RBAC with Owner/Admin/Member/Viewer, server-side enforcement.

**Independent Test**: Each role denied prohibited ops; UI reflects permissions but not relied on for security.

### Implementation for US5

- [x] T051 [P] [US5] Permission matrix and checks (crates/permissions)
- [x] T052 [P] [US5] Authorization middleware on API routes
- [x] T053 [US5] Web UI: hide/disable actions based on permissions
- [x] T054 [US5] Mobile UI: permission-aware UI
- [x] T055 [US5] Negative security tests (viewer cannot delete, etc., direct API calls)

**Checkpoint**: US5 complete.

---

## Phase 8: User Story 6 - Use a real working dashboard (Priority: P0)

**Goal**: Tenant-aware dashboard with real metrics and activity.

**Independent Test**: Dashboard shows real authorized data scoped to active tenant with loading/empty/error states.

### Implementation for US6

- [x] T056 [P] [US6] Dashboard queries (actual data from DB, tenant-scoped)
- [x] T057 [P] [US6] API endpoints for dashboard metrics and activity
- [x] T058 [US6] Web dashboard UI with cards, activity feed, states
- [x] T059 [US6] Mobile dashboard UI with optimized layouts
- [x] T060 [US6] Tests for dashboard data scoping

**Checkpoint**: US6 complete.

---

## Phase 9: User Story 7 - Manage records with CRUD workflows (Priority: P0)

**Goal**: Reusable search/filter/sort/paginate forms/lists/detail/CRUD with validation.

**Independent Test**: End-to-end CRUD on template entities works on both platforms with validation and auth.

### Implementation for US7

- [x] T061 [P] [US7] Generic list/pagination utilities
- [x] T062 [P] [US7] Form validation shared (contracts/app-core)
- [x] T063 [US7] Web CRUD components/patterns (ui-admin)
- [x] T064 [US7] Mobile CRUD screens/patterns
- [x] T065 [US7] CRUD API endpoints for members/invitations as demo
- [x] T066 [US7] Tests for CRUD flows, validation, confirmation

**Checkpoint**: US7 complete.

---

## Phase 10: User Story 8 - Use integrated AI chatbot (Priority: P0)

**Goal**: AI chatbot on web/mobile with conversation history, isolation, streaming, clear errors.

**Independent Test**: With configured provider, can send/receive messages, history persisted, tenant/user isolation enforced.

### Implementation for US8

- [x] T067 [P] [US8] AI core interfaces (ai-core crate) - messages, conversations, streaming
- [x] T068 [P] [US8] AI provider abstraction + OpenAI-compatible adapter (ai-providers)
- [x] T069 [P] [US8] Conversation persistence (DB models/tables)
- [x] T070 [P] [US8] API endpoints for conversations/messages (streaming via SSE)
- [x] T071 [US8] Web chatbot UI (ui-ai + integration)
- [x] T072 [US8] Mobile chatbot UI
- [x] T073 [US8] Tenant/user conversation isolation
- [x] T074 [US8] Error handling for provider unavailable
- [x] T075 [US8] Tests for conversation isolation and failures

**Checkpoint**: US8 complete.

---

## Phase 11: User Story 9 - Customize preferences and settings (Priority: P1)

**Goal**: Personal preferences (theme) and settings separation with proper auth.

**Independent Test**: Preferences persist across sessions/platforms; authorized settings only accessible.

### Implementation for US9

- [x] T076 [P] [US9] User preferences API and DB
- [x] T077 [US9] Profile/settings UI on web
- [x] T078 [US9] Profile/settings UI on mobile
- [x] T079 [US9] Theme switching with persistence
- [x] T080 [US9] Tests for preferences

**Checkpoint**: US9 complete.

---

## Phase 12: User Story 10 - Review notifications and audit history (Priority: P1)

**Goal**: In-app notifications and audit trail with tenant scope and authorization.

**Independent Test**: Admin actions produce auditable events; only authorized users see audit; no modification via API.

### Implementation for US10

- [x] T081 [P] [US10] Notifications model/API (read/unread, tenant scope)
- [x] T082 [P] [US10] Audit events model/API (append-only, protected)
- [x] T083 [US10] Web UI for notifications and audit
- [x] T084 [US10] Mobile UI for notifications and audit
- [x] T085 [US10] Emit audit events for key actions (auth, membership, role changes)
- [x] T086 [US10] Tests for audit authorization and immutability

**Checkpoint**: US10 complete.

---

## Phase 13: User Story 11 - Extend template with new modules (Priority: P1)

**Goal**: Documented extension process demonstrating adding module respecting all constraints.

**Independent Test**: Follow extension guide to add small generic resource module end-to-end.

### Implementation for US11

- [x] T087 [P] [US11] Create extension guide (docs/extension-guide.md) with concrete example
- [x] T088 [P] [US11] Implement example generic resource (module pattern) showing pages/endpoints/permissions
- [x] T089 [US11] Ensure example works on web and mobile with parity
- [x] T090 [US11] Tests for example module

**Checkpoint**: US11 complete.

---

## Phase 14: User Story 12 - Configure AI providers and approve actions (Priority: P1)

**Goal**: Provider abstraction allows swapping; consequential actions require explicit approval.

**Independent Test**: Swap provider behind interface without changing UI; approval-required tools blocked until approved.

### Implementation for US12

- [x] T091 [P] [US12] AI tool framework with typed definitions, validation, auth, tenant scope
- [x] T092 [P] [US12] Tool execution with approval workflow (requested/approved/denied)
- [x] T093 [P] [US12] AI tool audit logging (ai_tool_executions)
- [x] T094 [US12] Read-only initial tool set (dashboard metrics, authorized records)
- [x] T095 [US12] UI for approval (web/mobile) when tools require approval
- [x] T096 [US12] Prevent self-approval; enforce server-side approval
- [x] T097 [US12] Tests for approval flow, unauthorized access, invalid args

**Checkpoint**: US12 complete.

---

## Phase 15: Polish & Cross-Cutting Concerns

**Purpose**: Hardening, deployment docs, final validation.

- [x] T098 [P] Rust/UI evaluation POC and decision doc (docs/rust-ui-evaluation.md) per criteria
- [x] T099 [P] Deployment reference architecture (docs/deployment.md) with containerization, secrets, health
- [x] T100 [P] Feature parity matrix completion and validation
- [x] T101 [P] Security review checklist (auth, tenant isolation, AI safety, audit)
- [x] T102 Run all tests (`cargo test --workspace`) and ensure passing
- [x] T103 Run clippy/fmt checks
- [x] T104 Validate quickstart.md end-to-end
- [x] T105 Code cleanup, remove dead code, ensure docs consistent

**Checkpoint**: All complete. Ready for PR/release.

---

## Dependencies & Execution Order

### Phase Dependencies
- Phase 1 (Setup) → Phase 2 (Foundational) - Foundation blocks everything
- Phase 2 → All User Story Phases (3-14) - can run in parallel once foundational complete
- All User Stories (3-14) → Phase 15 (Polish)

### User Story Priority
P0 stories: US1-US8 (Phases 3-10) - core functionality. P1: US9-US12 (Phases 11-14).

### Parallel Opportunities
- Setup tasks T001-T008 mostly parallel where different files
- Foundational T009-T024: many parallel (different crates/apps)
- Within each user story, tasks marked [P] are parallel
- Different user stories can proceed in parallel once Phase 2 complete (team capacity)

## Parallel Example: Foundational Phase
Launch in parallel: T009, T010, T011, T012, T013, T014, T015, T016, T017, T018, T020, T021, T024 (different crates/migrations/tests). T019,T022,T023 sequential where they depend on others.

## MVP Scope
MVP = Phases 1-10 (US1 through US8): setup, foundation, auth, parity, tenancy/roles, dashboard, CRUD, AI chatbot. This delivers core template functionality.
