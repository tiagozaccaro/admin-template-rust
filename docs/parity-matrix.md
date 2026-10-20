# Feature Parity Matrix

| Feature | Web | Mobile | Notes |
|---|---|---|---|
| Authentication (Google OIDC) | ✓ | ✓ | Shared auth API and session semantics |
| Multi-tenancy | ✓ | ✓ | Server-enforced tenant isolation |
| RBAC (Owner/Admin/Member/Viewer) | ✓ | ✓ | Shared permission model |
| Dashboard (metrics/activity) | ✓ | ✓ | Shared contracts via API |
| CRUD Operations | ✓ | ✓ | Same validation on server |
| AI Chatbot | ✓ | ✓ | Shared API, streaming via SSE |
| Notifications | ✓ | ✓ | In-app notifications per tenant/user |
| Audit Trail | ✓ | ✓ | Admin-only, append-only |
| User Preferences | ✓ | ✓ | Theme/locale/settings |
| AI Tools & Approval | ✓ | ✓ | Server-side approval workflow |
| Invitations | ✓ | ✓ | Tenant membership invitations |

**Parity principle**: All supported capabilities share the same data model, business rules, and auth semantics. Platform-specific code is limited to navigation, secure storage, and platform auth adapters only.
