# Research: Rust-First Admin Template

**Date**: 2026-10-18  
**Feature**: 002-rust-admin-template  
**Status**: Phase 0 Complete

## Research Questions

1. Dioxus version and mobile targets (Android/iOS initial support)
2. Rust/UI component registry evaluation (compatibility, license, maintenance, accessibility, theming)
3. OIDC library and session strategy (Google OIDC)
4. Initial AI provider and capabilities
5. Background jobs mechanism
6. Email/invitation delivery
7. Deployment target and secret management
8. Crate boundaries (post-POC refinement)

## 1) Dioxus Version and Mobile Targets

### Decision
- Use **Dioxus v0.6.x** (current stable 0.6 series). This version provides stable web and mobile (iOS/Android) support with improved ergonomics and tooling.
- Initial mobile targets: **Android** and **iOS** (explicitly supported). Desktop targets not required for initial template; focus on web + mobile parity as specified.

### Rationale
- 0.6.x is the actively maintained release line with documented mobile workflows (dioxus-cli, cargo-mobile tooling integration patterns). 
- Matches the spec requirement to verify mobile targets and build on both platforms.
- Broad ecosystem compatibility for the tooling we need (bundling, dev server, hot reload).

### Alternatives Considered
- Dioxus 0.5.x: older, less mature mobile ergonomics; some ecosystem crates lagging.
- Dioxus 0.7+ (pre/stable when evaluated): may have breaking changes; better to pin to stable 0.6.x for template stability and predictable upgrades.

### Implementation Notes
- Use `dioxus`/`dioxus-web`/`dioxus-mobile` crates aligned to same 0.6.x version.
- Use `dioxus-cli` for dev/build workflows. Document platform-specific prerequisites (Android SDK, Xcode) in README.
- Configure feature flags appropriately per platform.

## 2) Rust/UI Component Registry Evaluation

### Decision
- **Evaluate `dioxus-components` / community UI kits** and also consider `daisyui`-inspired CSS approach via Dioxus + Tailwind/CSS modules. For initial POC, implement minimal shared components in `crates/ui` (button, input, checkbox, dialog/sheet, select/dropdown, tabs, table/list, toast, layout, theme switch) to avoid premature third-party coupling. 
- If a specific Rust/UI registry passes all criteria (compat 0.6.x, web+mobile, license, maintenance, accessibility, theming, styling build reqs, component coverage), we may adopt/adapt. Otherwise, use internal `crates/ui` as the canonical component source.

### Evaluation Criteria (from spec)
1. Compatibility with chosen Dioxus version (0.6.x)
2. Web build/runtime compatibility
3. Mobile build/runtime compatibility (Android/iOS)
4. License and redistribution terms
5. Maintenance activity and upgrade risks
6. Accessibility and keyboard/focus behavior
7. Theme and styling customization
8. Tailwind and Node/pnpm build-tool requirements
9. Component completeness for admin interfaces
10. Source ownership/modifiability
11. AI-assisted development suitability

### POC Plan
- Build minimal set: Button, Icon, Input + validation, Checkbox, Dialog/Sheet, Select, Tabs, Table/List (mobile-appropriate), Toast, Responsive layout, Theme switch.
- Render same components on web and mobile; verify styling, focus/keyboard, theming (light/dark), responsive behavior.

### Rationale
- Keeps control over accessibility, theming, and avoids pulling unnecessary deps. Also easier to enforce "shared UI components must not require web runtime on mobile" and consistent API.
- Many "Rust UI" registries vary in Dioxus 0.6 support; internal components are safe fallback.

### Alternatives
- `dioxus-ui` ecosystem crates: need vetting per criteria.
- Tailwind CSS (via CLI/postcss) for styling with headless component patterns.

## 3) OIDC Library and Session Strategy

### Decision
- Use **`openidconnect`** (Rust OIDC/OAuth2) for server-side OIDC flows. Complement with **`oauth2`** crate as needed.
- Session strategy: **server-side sessions** stored in PostgreSQL (session store table) with secure HTTP-only cookies for web clients. For mobile clients, use **Authorization header with bearer tokens** (opaque session tokens or short-lived access tokens + refresh tokens) as mobile apps cannot rely on same cookie semantics; implement platform-appropriate storage (secure keystore/keychain). 
- PKCE required for public clients (mobile). Use authorization code flow with state/nonce validation.

### Rationale
- `openidconnect` is well-maintained, standards-compliant, works with Google OIDC. Avoid rolling crypto.
- Hybrid session model (cookies for web, tokens for mobile) matches platform constraints noted in spec.

### Alternatives
- `auth0`/third-party SDKs: lock-in, overkill for template.
- JWT-only stateless sessions: harder to revoke; server-side sessions simpler for revocation and audit.

### Security Notes
- Validate issuer/audience, signatures, exp/iat, nonce/state. Prevent silent email merging (key by provider+subject). Keep client secrets server-side only.

## 4) Initial AI Provider and Capabilities

### Decision
- Initial provider: **OpenAI-compatible API** (configurable base URL, API key). Default to a widely available model; make provider and model fully configurable via env/config.
- Support: text generation, streaming (SSE where suitable), timeouts/cancellation, token usage metadata if available, error handling, rate limits. Provider abstraction isolates vendor-specific formats.

### Rationale
- Broad compatibility (OpenAI, local models via Ollama/vllm, Azure OpenAI-compatible endpoints) - aligns with "provider-independent" requirement. Easy to add more providers later.
- Good Rust ecosystem support for OpenAI-compatible APIs; streaming is well-defined.

### Alternatives
- Anthropic: also good; OpenAI-compatible gives more flexibility for self-host/local dev.
- Multiple providers from day one: adds complexity before proving abstraction.

### Abstraction
- `ai-core`: traits (ChatProvider, StreamChunk, Tool, etc.), message contracts, tool definitions/execution interfaces.
- `ai-providers`: concrete adapter(s) implementing traits.

## 5) Background Jobs Mechanism

### Decision
- **Start simple**: use `tokio::spawn` with a job queue in-process + persistence for job state (Postgres table) if needed. Introduce a separate `apps/worker` binary only when actual background processing is required (invitations, notifications, cleanup, AI tasks). 
- Implement job traits with retry, idempotency keys, max attempts, backoff, failure tracking, and structured logging.

### Rationale
- Avoid premature infra (Redis/sidekiq-like systems). In-process is fine for template scope initially; can evolve to out-of-process/queue backend later.
- Spec states: "A separate worker application SHOULD be introduced when actual background processing requires it, not merely to fill a proposed folder."

### Alternatives
- `sqlxmq`, `lapin` (RabbitMQ), Redis-backed queues: add operational complexity. Defer until concrete jobs exist.

## 6) Email/Invitation Delivery

### Decision
- Make invitation delivery **pluggable** via a `NotificationSender`/`EmailSender` trait. 
- Initial implementation: **logging-only/no-op sender** in dev (logs the invitation link/token). If SMTP config is provided, implement SMTP sender (e.g. `lettre` crate). 
- System must clearly indicate delivery state (sent/logged/failed) and not pretend delivery when unconfigured - matches spec.

### Rationale
- Keeps local dev simple (no email server required). Production can configure SMTP/transactional email provider later.

### Alternatives
- SendGrid/Mailgun SDKs: vendor-specific; trait keeps flexibility.

## 7) Deployment Target and Secret Management

### Decision
- Document **containerized deployment** (Docker multi-stage builds for api/web assets). Target platforms: **Fly.io**, **Render**, or **Railway** as common options; also document generic Docker + systemd/Kubernetes notes. Pick one primary example (Fly.io/Render) in quickstart docs.
- Secret management: use **environment variables** for local dev (.env with example, never commit real secrets). For production, use platform secret stores (Fly secrets, Render env vars, Vault if self-hosted). Never embed secrets in web/mobile bundles.
- Health/readiness endpoints in API.

### Rationale
- Containerization is portable and matches "documented deployment reference architecture". Env vars are simple and standard; platform secret stores are secure.

### Alternatives
- Cloud-specific (AWS only): reduces portability. Better to provide generic containerized approach + one concrete example.

## 8) Crate Boundaries (Refinement Plan)

### Initial Proposed Boundaries (to refine post-Phase 0 POC)
- `contracts`: API DTOs shared by api, api-client, web, mobile (serde + types). Keep backend-specific types separate.
- `app-core`: domain logic, use cases, validation rules shared where pure (no I/O). Backend authority lives here.
- `auth`: auth abstractions, session, OIDC integration helpers.
- `permissions`: RBAC policy, permission checks, role matrix.
- `tenancy`: tenant/membership/invitation domain logic.
- `database`: SQLx models/repositories, migrations, connection pool.
- `ai-core`: provider traits, tool interfaces, message types.
- `ai-providers`: concrete provider adapters.
- `ai-agents`: tool registry, execution policy, approval/audit integration.
- `config`: validated config (figment/env).
- `observability`: tracing/logging middleware, request IDs.
- `ui/*`: shared UI primitives/patterns (Dioxus components). Ensure no web-only APIs leak.
- `api-client`: typed client for web/mobile to call API.

### Post-POC Refinement Criteria
- Minimize circular deps (enforce via workspace; consider visibility).
- Keep UI platform-agnostic where possible; platform adapters separate.
- Avoid premature abstraction - only extract when reused or needed for testability.
- Ensure contracts remain the single source of truth for API shape.

## Phase 0 Completion

All open decisions are resolved with rationale and alternatives. Research findings are sufficient to proceed to Phase 1 (data model, contracts, quickstart). No unresolved NEEDS CLARIFICATION remain from the plan.

