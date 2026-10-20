# API Contracts (Draft)

**Date**: 2026-10-18  
**Feature**: 002-rust-admin-template

## Overview

Versioned REST/JSON API with typed contracts. All protected endpoints require authentication, enforce tenant context, and return consistent error responses. Request IDs included via middleware. OpenAPI/contract documentation generated from types where feasible.

## Base Types

### Error Response
```json
{
  "error": {
    "code": "string",
    "message": "string",
    "details": { "optional": true }
  },
  "request_id": "uuid"
}
```

### Pagination
```json
{
  "items": [...],
  "page": 1,
  "per_page": 20,
  "total": 100,
  "has_next": true
}
```

## Auth

### POST /api/v1/auth/login/google/start
Start Google OAuth flow. Returns auth URL or redirect handled client-side.
Response: `{ "auth_url": "string" }`

### GET /api/v1/auth/callback/google
OAuth callback. Exchanges code for tokens, validates state/nonce, creates session.
Returns session info + redirect. For web, sets secure HTTP-only cookie.

### POST /api/v1/auth/logout
Revokes current session.
Response: `{ "success": true }`

### GET /api/v1/auth/me
Current authenticated user + session context + active tenant.
Response: `{ "user": {...}, "session": {...}, "active_tenant": {...} | null }`

## Tenants & Memberships

### GET /api/v1/tenants
List tenants user belongs to.
Query: pagination optional.

### POST /api/v1/tenants
Create tenant (authenticated). Initializes owner membership.
Body: `{ "name": "string", "slug": "string?" }`

### GET /api/v1/tenants/:id
Get tenant details (member required).

### PATCH /api/v1/tenants/:id
Update tenant (owner/admin per permissions).

### POST /api/v1/tenants/:id/switch
Switch active tenant in session.

### GET /api/v1/tenants/:id/members
List members (member required).

### POST /api/v1/tenants/:id/invitations
Invite user by email (admin+). 
Body: `{ "email": "string", "role": "admin|member|viewer" }`
Response includes delivery status.

### GET /api/v1/invitations/:token
View invitation details.

### POST /api/v1/invitations/:token/accept
Accept invitation (creates membership if valid).

### POST /api/v1/invitations/:token/reject
Reject invitation.

### DELETE /api/v1/tenants/:id/members/:user_id
Remove member (admin+, with safeguards).

## Users & Profile

### GET /api/v1/profile
Current user profile.

### PATCH /api/v1/profile
Update profile (display name, etc.).

### GET /api/v1/profile/preferences
Get user preferences.

### PATCH /api/v1/profile/preferences
Update preferences (theme, locale).

## Dashboard

### GET /api/v1/dashboard/metrics
Tenant-scoped dashboard metrics (real data).
Query: date range optional.

### GET /api/v1/dashboard/activity
Recent activity feed (tenant-scoped).

## Settings

### GET /api/v1/tenants/:id/settings
Get tenant settings (authorized).

### PATCH /api/v1/tenants/:id/settings
Update tenant settings (authorized). Secrets never exposed.

## Notifications & Audit

### GET /api/v1/notifications
List in-app notifications for current user in active tenant (paginated).

### POST /api/v1/notifications/:id/read
Mark as read.

### GET /api/v1/audit
List audit events (admin+). Tenant-scoped. Paginated, append-only.

## AI Conversations

### GET /api/v1/ai/conversations
List conversations (paginated, tenant-scoped).

### POST /api/v1/ai/conversations
Create new conversation.
Body: `{ "title": "string?", "provider": "string?", "model": "string?" }`

### GET /api/v1/ai/conversations/:id
Get conversation + messages.

### POST /api/v1/ai/conversations/:id/messages
Send message. Supports streaming via SSE if enabled.
Body: `{ "content": "string" }`
Returns message + possibly stream URL or streamed response.

### DELETE /api/v1/ai/conversations/:id
Archive/delete per policy.

## AI Tools

### GET /api/v1/ai/tools
List available tools (filtered by permissions).

### POST /api/v1/ai/tools/:tool_name/execute
Execute tool (server validates auth, tenant scope, approval). 
Body: `{ "input": {...}, "conversation_id": "uuid?", "message_id": "uuid?" }`
If approval required, returns pending status.

### POST /api/v1/ai/tools/executions/:id/approve
Approve pending tool execution (authorized approver).

### POST /api/v1/ai/tools/executions/:id/deny
Deny pending tool execution.

## Health

### GET /healthz
Liveness.

### GET /readyz
Readiness (DB connectivity, etc.).
