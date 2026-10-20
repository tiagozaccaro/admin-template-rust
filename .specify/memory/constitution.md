<!-- Sync Impact Report
Version change: (scaffold) → 1.0.0
Added sections: Preamble, Article I (Rust-First Technology), Article II (Web and Mobile Feature Parity), Article III (Reusable and Modular Architecture), Article IV (Design System and Rust/UI Evaluation), Article V (Authentication and Identity), Article VI (Security, Privacy, and Authorization), Article VII (Multi-Tenancy and Roles), Article VIII (AI Provider Independence and Safe Execution), Article IX (Testing and Quality), Article X (Accessibility and User Experience), Article XI (Observability and Operations), Article XII (Documentation and Genericity), Article XIII (Spec-Driven Development), Article XIV (Pull Requests and Release Governance), Article XV (Governance and Amendments), Compliance Review
Removed sections: Generic placeholder principles (PRINCIPLE_1-5), generic SECTION_2/SECTION_3
Modified principles: Replaced template placeholders with concrete, testable governance articles
Follow-up TODOs: None (all placeholders replaced)
-->

# Admin Template Constitution

**Version:** 1.0.0  
**Status:** Initial constitution  
**Project:** `admin-template`

## Preamble

Admin Template is a reusable, production-oriented, Rust-first foundation for building administrative applications. It provides a shared product foundation across web and mobile, secure authentication and authorization, multi-tenant administration, a consistent design system, and provider-independent AI capabilities.

The template must remain generic. It must not contain business-specific workflows, product assumptions, or dependencies on any previous application.

This constitution governs architecture, implementation, testing, security, delivery, and maintenance. All specifications, plans, tasks, and pull requests MUST comply with it.

## Article I — Rust-First Technology

1. Rust MUST be the primary language for application logic, shared domain logic, API services, authorization, and AI integration.
2. Dioxus MUST be the primary UI framework for both web and mobile.
3. The backend MUST use Rust with Axum and Tokio unless an approved technical decision establishes a compelling reason to change.
4. PostgreSQL MUST be the default relational database, accessed through SQLx and versioned migrations.
5. Shared application contracts MUST use strongly typed Rust structures and explicit serialization.
6. Node.js, pnpm, CSS tooling, platform-specific native integrations, and other non-Rust tools MAY be used when required by the selected UI framework or build ecosystem. They MUST remain scoped to their purpose and MUST NOT become a second application-logic stack without documented justification.
7. Introducing another primary UI framework, backend language, or database MUST require an explicit architecture decision and documented trade-off analysis.

## Article II — Web and Mobile Feature Parity

1. Web and mobile MUST expose the same supported product capabilities, business rules, authorization rules, and data operations.
2. Platform-specific presentation is encouraged. Identical screens are not required.
3. Differences in navigation, information density, input methods, dialogs, tables, lists, and responsive layouts MAY be implemented for each platform.
4. A feature MUST NOT be considered complete until its required web and mobile behavior is implemented and verified, unless the specification explicitly identifies a justified platform limitation.
5. Shared domain rules, contracts, validation, permissions, and API behavior MUST NOT be independently reimplemented in ways that can diverge.
6. Feature parity MUST be documented in a maintained matrix covering routes or screens, operations, roles, loading states, errors, and acceptance tests.
7. Automated tests MUST detect significant differences in supported functionality and authorization between platforms.

## Article III — Reusable and Modular Architecture

1. The system MUST use clear boundaries between UI components, application services, domain logic, API contracts, persistence, authentication, authorization, and AI infrastructure.
2. The default backend architecture SHOULD be a modular monolith. Microservices MUST NOT be introduced without a documented operational or scaling requirement.
3. Reusable code MUST live in shared modules or crates with explicit public interfaces.
4. UI components MUST be reusable, accessible, configurable, and independent of application-specific business logic.
5. Platform adapters MAY implement presentation and native integrations, but MUST delegate business behavior to shared services or backend APIs.
6. The template MUST remain suitable for cloning and extending into unrelated products.
7. Features MUST be configurable where practical rather than hard-coded to a particular deployment, tenant, AI provider, or external service.

## Article IV — Design System and Rust/UI Evaluation

1. Dioxus-compatible Rust/UI components MUST be evaluated during the initial feasibility phase.
2. The evaluation MUST verify framework compatibility, current maintenance status, license compatibility, accessibility, component coverage, customization, styling/build requirements, and web/mobile support.
3. The project MUST prove that representative components work in actual web and mobile builds before adopting the registry as a foundational dependency.
4. Components SHOULD be copied into the project source when the registry's distribution model supports that approach, enabling customization and reducing unnecessary runtime dependencies.
5. Registry components MUST NOT be treated as a substitute for application architecture, authentication, authorization, backend services, or domain logic.
6. If Rust/UI fails the acceptance criteria, the team MUST document the findings and choose a maintainable Dioxus-compatible alternative.
7. The UI MUST provide consistent typography, spacing, colors, responsive behavior, focus states, validation feedback, and light/dark theme support where practical.

## Article V — Authentication and Identity

1. Authentication MUST be implemented through established standards and well-maintained libraries rather than custom cryptographic protocols.
2. Google OpenID Connect MUST be the first supported external login provider.
3. The authentication architecture MUST allow additional identity providers to be added without rewriting application authorization or user-management logic.
4. The system MUST distinguish external identity from internal user identity. External accounts MUST be keyed by provider and stable provider subject identifier, not email alone.
5. Provider credentials and client secrets MUST remain server-side.
6. Login, callback, logout, session renewal, account linking, and error handling MUST be explicitly designed and tested.
7. Account linking MUST require proof of control of the accounts being linked. Matching email addresses alone MUST NOT silently merge identities.
8. Web and mobile MUST use appropriate OAuth/OIDC flows, redirect handling, state validation, nonce validation where applicable, and PKCE where appropriate.
9. Authentication MUST NOT imply authorization. Every protected operation MUST enforce its authorization requirements independently.

## Article VI — Security, Privacy, and Authorization

1. Security MUST be a design requirement, not a final hardening activity.
2. Every API operation that accesses protected data MUST enforce authentication, authorization, and tenant boundaries on the server.
3. Client-side route guards and hidden UI controls MUST NOT be considered security controls.
4. Input MUST be validated at trust boundaries. Database access MUST use parameterized queries.
5. Secrets MUST be supplied through secure configuration or secret-management facilities and MUST NOT be committed to source control.
6. Web sessions MUST use appropriately secured cookies or another explicitly justified secure mechanism. Cookie-based state-changing operations MUST have appropriate CSRF protection.
7. Authentication and authorization failures MUST not disclose sensitive internal details.
8. Logs MUST NOT contain passwords, access tokens, refresh tokens, provider secrets, or unnecessary personal or financial data.
9. Tenant isolation, membership changes, role changes, account linking, and AI tool execution MUST receive explicit security tests.
10. Destructive or externally consequential operations MUST have clear authorization and confirmation requirements.

## Article VII — Multi-Tenancy and Roles

1. The template MUST support users belonging to multiple organizations or tenants, and tenants containing multiple users.
2. Tenant membership MUST be explicit and independently managed from global user identity.
3. The default role model MUST include at least Owner, Admin, Member, and Viewer, with documented permissions and room for extension.
4. The active tenant MUST be resolved and validated by trusted server-side logic.
5. Every tenant-owned record and operation MUST enforce tenant scope, including background jobs, exports, searches, notifications, and AI retrieval.
6. Tenant context MUST NOT be trusted merely because it was supplied by a client.
7. Cross-tenant access MUST fail closed and be covered by automated tests.
8. The template MUST support a clear tenant-switching experience on web and mobile.

## Article VIII — AI Provider Independence and Safe Execution

1. AI functionality MUST be implemented behind a provider-independent interface.
2. Provider-specific SDKs, request formats, model identifiers, and streaming mechanisms MUST be isolated behind adapters.
3. Providers and models MUST be configurable without changing the shared chatbot UI or core application domain.
4. AI requests MUST support appropriate timeouts, cancellation, bounded retries, rate limits, usage limits, and useful error reporting.
5. The architecture MUST account for streaming responses and conversation persistence.
6. AI responses MUST be treated as untrusted content. They MUST NOT bypass server-side authorization, tenant boundaries, or data-access rules.
7. AI tools MUST use explicit schemas, validated arguments, least-privilege permissions, bounded execution, and auditable results.
8. Read-only AI tools MAY run when authorized. Destructive, financial, administrative, or externally consequential actions MUST require explicit user approval before execution.
9. AI-generated content MUST NOT be rendered as trusted HTML or executable code without appropriate sanitization and isolation.
10. Model providers MUST receive only the data necessary for an authorized request. Data retention and provider-specific privacy behavior MUST be documented.
11. If no provider is configured, the application MUST remain usable and present a clear, non-crashing AI-unavailable state.

## Article IX — Testing and Quality

1. New behavior MUST include tests appropriate to its risk and architecture.
2. Shared domain logic MUST have unit tests.
3. API endpoints, persistence, authentication, authorization, tenant isolation, and AI tool execution MUST have integration tests where applicable.
4. Critical user journeys MUST have end-to-end coverage.
5. Web and mobile MUST each receive platform-level verification for their supported features.
6. Rust code MUST pass formatting and lint checks, including `cargo fmt --check` and Clippy with the repository's configured warnings policy.
7. Builds and tests MUST be reproducible from documented setup instructions.
8. Tests MUST NOT depend on real production credentials or uncontrolled paid AI calls.
9. Security-sensitive changes MUST include explicit negative tests, not only successful-path tests.
10. A task MUST NOT be declared complete merely because it compiles.

## Article X — Accessibility and User Experience

1. The UI MUST provide meaningful loading, success, empty, error, and disabled states.
2. Forms MUST have accessible labels, understandable validation messages, keyboard support where applicable, and clear focus behavior.
3. Web layouts MUST support practical desktop and narrow-screen use.
4. Mobile layouts MUST account for touch targets, platform navigation, smaller screens, keyboard behavior, and interrupted network requests.
5. Data tables MAY become searchable lists or detail views on mobile, but MUST preserve supported operations and permission rules.
6. Color MUST NOT be the only means of communicating status.
7. The application MUST provide a coherent theme and reusable interaction patterns.

## Article XI — Observability and Operations

1. Services MUST emit structured, useful logs with appropriate severity.
2. Errors MUST be observable without leaking secrets or unnecessary user data.
3. Distributed operations SHOULD include correlation or request identifiers.
4. External provider failures, background-job failures, and AI usage SHOULD be measurable.
5. Health and readiness checks MUST distinguish application availability from dependency readiness where appropriate.
6. Configuration MUST be validated at startup when invalid configuration would prevent safe operation.

## Article XII — Documentation and Genericity

1. The repository MUST document architecture, local development, configuration, database migrations, testing, deployment, and troubleshooting.
2. Every required environment variable MUST have a documented purpose and safe example.
3. External authentication setup MUST document Google OAuth/OIDC configuration and redirect URLs.
4. AI setup MUST document provider configuration, supported capabilities, and behavior when providers are unavailable.
5. New components, modules, public interfaces, and non-obvious security decisions MUST be documented.
6. Product-specific sample data MUST be clearly identified as demo data and MUST NOT be required for the template to work.
7. Removing or changing a foundational dependency MUST include an impact assessment.

## Article XIII — Spec-Driven Development

1. GitHub Spec Kit MUST be the primary workflow for defining and implementing substantial features.
2. Requirements MUST be established before implementation tasks are generated.
3. Ambiguous requirements MUST be clarified before irreversible architectural decisions are made.
4. The constitution, specification, plan, tasks, tests, and implementation MUST remain consistent.
5. Work MUST be divided into small, reviewable batches with explicit acceptance criteria.
6. The agent MUST inspect the existing repository before modifying it and MUST preserve valid existing work unless change is explicitly required.
7. Agents MUST NOT silently invent product requirements or bypass security and platform-parity requirements.
8. Every implementation batch MUST identify affected features, tests, documentation, and any architectural decisions.

## Article XIV — Pull Requests and Release Governance

1. Every implementation batch MUST be delivered through a branch and pull request unless the repository explicitly defines an approved alternative.
2. Pull requests MUST identify the specification tasks or issues they complete.
3. CI MUST pass, required review MUST be approved, and the pull request MUST be merged before dependent work is treated as complete.
4. The feature queue MUST NOT advance based solely on an agent's claim that implementation is finished.
5. Changes to authentication, authorization, tenancy, AI tool execution, or data migrations MUST receive appropriate focused review.
6. Schema changes MUST include versioned migrations and compatibility or rollout considerations.
7. Failed CI, rejected review, or unresolved acceptance criteria MUST block completion and downstream work.
8. Implementation agents MUST use bounded retries and report unresolved failures rather than repeatedly applying unverified fixes.

## Article XV — Governance and Amendments

1. The constitution is the highest-level project engineering policy.
2. Specifications, plans, tasks, code, and CI rules MUST NOT contradict it without an explicit amendment.
3. Amendments MUST state the reason, impact, and migration implications.
4. Material architectural exceptions MUST be documented in an architecture decision record or equivalent repository document.
5. When requirements conflict, security, tenant isolation, data integrity, and authorization take precedence over convenience or implementation speed.

## Compliance Review

Before a feature is approved, reviewers MUST verify:

- The feature satisfies its specification and acceptance criteria.
- Shared behavior remains consistent between web and mobile.
- Authentication, permissions, and tenant boundaries are enforced server-side.
- AI behavior follows provider-independence and safety requirements.
- Tests, formatting, linting, documentation, and CI are complete.
- No unapproved architectural exception or product-specific coupling has been introduced.

**Constitution status:** Active once adopted through the repository's Spec Kit constitution workflow.  
**Version**: 1.0.0 | **Ratified**: 2026-10-18 | **Last Amended**: 2026-10-18
