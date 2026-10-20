# Data Model: Rust-First Admin Template

**Date**: 2026-10-18  
**Feature**: 002-rust-admin-template

## Overview

Core entities supporting authentication, multi-tenancy, RBAC, notifications, audit, and AI conversations/tools. All tenant-owned entities are scoped to `tenant_id`. Authorization and tenant isolation enforced server-side.

## Entities

### users
Stores internal user identity.

- `id` (uuid, PK)
- `email` (text, unique, indexed) - canonical email
- `display_name` (text, nullable)
- `avatar_url` (text, nullable)
- `created_at` (timestamptz)
- `updated_at` (timestamptz)
- `last_login_at` (timestamptz, nullable)

**Constraints**: unique email. No secrets stored here.

### external_identities
Links external OIDC identities to internal users. Prevents silent merging (composite unique on provider + provider_subject).

- `id` (uuid, PK)
- `user_id` (uuid, FK -> users.id, cascade delete)
- `provider` (text) - e.g., "google"
- `provider_subject` (text) - stable subject from OIDC
- `provider_email` (text, nullable)
- `provider_claims` (jsonb, nullable) - safe subset only
- `linked_at` (timestamptz)
- `created_at` (timestamptz)
- `updated_at` (timestamptz)

**Constraints**: UNIQUE(provider, provider_subject). FK to users. Index (user_id), (provider_email).

### sessions
Server-side sessions for authenticated users.

- `id` (uuid, PK) - opaque session id
- `user_id` (uuid, FK -> users.id, cascade delete)
- `active_tenant_id` (uuid, FK -> tenants.id, nullable)
- `expires_at` (timestamptz, indexed)
- `revoked_at` (timestamptz, nullable, indexed)
- `created_at` (timestamptz)
- `last_accessed_at` (timestamptz)
- `ip_address` (inet, nullable)
- `user_agent` (text, nullable)
- `client_type` (text, nullable) - "web" | "mobile"

**Constraints**: index (user_id), (expires_at) for cleanup. Soft revoke via revoked_at.

### tenants
Organizations/tenants (multi-tenant boundary).

- `id` (uuid, PK)
- `name` (text)
- `slug` (text, unique, indexed)
- `owner_id` (uuid, FK -> users.id)
- `settings` (jsonb, nullable)
- `created_at` (timestamptz)
- `updated_at` (timestamptz)
- `archived_at` (timestamptz, nullable)

**Constraints**: unique slug. Index (owner_id).

### tenant_memberships
User membership in tenants with roles.

- `id` (uuid, PK)
- `tenant_id` (uuid, FK -> tenants.id, cascade delete)
- `user_id` (uuid, FK -> users.id, cascade delete)
- `role` (text) - "owner" | "admin" | "member" | "viewer"
- `joined_at` (timestamptz)
- `invited_by` (uuid, FK -> users.id, nullable)
- `created_at` (timestamptz)
- `updated_at` (timestamptz)

**Constraints**: UNIQUE(tenant_id, user_id). Index (user_id), (tenant_id).

### invitations
Pending invitations to join tenants.

- `id` (uuid, PK)
- `tenant_id` (uuid, FK -> tenants.id, cascade delete)
- `email` (text, indexed)
- `role` (text) - "admin" | "member" | "viewer" (owner cannot be invited)
- `token` (uuid/text, unique) - opaque invitation token
- `invited_by` (uuid, FK -> users.id)
- `expires_at` (timestamptz, indexed)
- `accepted_at` (timestamptz, nullable)
- `revoked_at` (timestamptz, nullable)
- `delivery_status` (text, nullable) - "sent" | "logged" | "failed" | "pending"
- `delivery_error` (text, nullable)
- `created_at` (timestamptz)

**Constraints**: unique token. Index (tenant_id, email), (expires_at). Valid if accepted_at null and revoked_at null and expires_at > now.

### user_preferences
Per-user preferences.

- `id` (uuid, PK)
- `user_id` (uuid, FK -> users.id, cascade delete, unique)
- `theme` (text, nullable) - "light" | "dark" | "system"
- `locale` (text, nullable)
- `preferences` (jsonb, nullable)
- `created_at` (timestamptz)
- `updated_at` (timestamptz)

**Constraints**: one-to-one with users.

### tenant_settings
Per-tenant settings.

- `id` (uuid, PK)
- `tenant_id` (uuid, FK -> tenants.id, cascade delete, unique)
- `name` (text, nullable)
- `branding` (jsonb, nullable)
- `settings` (jsonb, nullable)
- `created_at` (timestamptz)
- `updated_at` (timestamptz)

**Constraints**: one-to-one with tenants.

### notifications
In-app notifications with tenant scope.

- `id` (uuid, PK)
- `tenant_id` (uuid, FK -> tenants.id, cascade delete)
- `user_id` (uuid, FK -> users.id, cascade delete)
- `title` (text)
- `body` (text, nullable)
- `read_at` (timestamptz, nullable, indexed)
- `created_at` (timestamptz, indexed)
- `expires_at` (timestamptz, nullable)
- `metadata` (jsonb, nullable)

**Constraints**: index (tenant_id, user_id, read_at), (created_at).

### audit_events
Immutable audit trail for security-relevant actions.

- `id` (uuid, PK)
- `tenant_id` (uuid, FK -> tenants.id, nullable) - may be null for pre-tenant actions
- `actor_user_id` (uuid, FK -> users.id, nullable)
- `action` (text) - namespaced action (e.g., "auth.login.success", "tenant.member.remove")
- `target_type` (text, nullable)
- `target_id` (uuid, nullable)
- `ip_address` (inet, nullable)
- `user_agent` (text, nullable)
- `summary` (text) - safe, non-sensitive
- `metadata` (jsonb, nullable) - only non-sensitive fields
- `created_at` (timestamptz, indexed)

**Constraints**: immutable (no updates/deletes via app API). Index (tenant_id, created_at), (actor_user_id), (action).

### ai_conversations
AI chat conversations with tenant/user scope.

- `id` (uuid, PK)
- `tenant_id` (uuid, FK -> tenants.id, cascade delete)
- `user_id` (uuid, FK -> users.id, cascade delete)
- `title` (text, nullable)
- `provider` (text, nullable)
- `model` (text, nullable)
- `status` (text) - "active" | "archived"
- `metadata` (jsonb, nullable)
- `created_at` (timestamptz)
- `updated_at` (timestamptz)
- `last_message_at` (timestamptz, nullable)

**Constraints**: index (tenant_id, user_id), (created_at).

### ai_messages
Messages within conversations.

- `id` (uuid, PK)
- `conversation_id` (uuid, FK -> ai_conversations.id, cascade delete)
- `role` (text) - "user" | "assistant" | "system"
- `content` (text) - message content
- `tool_calls` (jsonb, nullable)
- `token_usage` (jsonb, nullable)
- `metadata` (jsonb, nullable)
- `created_at` (timestamptz)

**Constraints**: index (conversation_id, created_at).

### ai_tool_executions
Auditable AI tool execution records.

- `id` (uuid, PK)
- `tenant_id` (uuid, FK -> tenants.id, cascade delete)
- `conversation_id` (uuid, FK -> ai_conversations.id, nullable, cascade delete)
- `message_id` (uuid, FK -> ai_messages.id, nullable, cascade delete)
- `user_id` (uuid, FK -> users.id, cascade delete)
- `tool_name` (text)
- `tool_input` (jsonb) - validated args (non-sensitive)
- `tool_output` (jsonb, nullable) - safe output only (no secrets)
- `status` (text) - "requested" | "approved" | "denied" | "executing" | "succeeded" | "failed" | "cancelled" | "timed_out"
- `approval_required` (boolean)
- `approved_by` (uuid, FK -> users.id, nullable)
- `approved_at` (timestamptz, nullable)
- `denied_reason` (text, nullable)
- `error_message` (text, nullable)
- `duration_ms` (int, nullable)
- `expires_at` (timestamptz, nullable)
- `created_at` (timestamptz)
- `completed_at` (timestamptz, nullable)

**Constraints**: index (tenant_id, created_at), (conversation_id), (tool_name, status).

## Relationships

- users 1:N external_identities
- users 1:N sessions, tenant_memberships, notifications, ai_conversations
- users 1:1 user_preferences
- tenants 1:N tenant_memberships, invitations, notifications, audit_events, ai_conversations, ai_tool_executions
- tenants 1:1 tenant_settings
- ai_conversations 1:N ai_messages
- ai_messages 1:N ai_tool_executions (nullable)
- ai_conversations 1:N ai_tool_executions

## Validation Rules

- Role values restricted to enum set above.
- Invitation role cannot be "owner".
- External identity: no silent merge - linking requires explicit verification.
- Tenant scope required for all tenant-owned entities on protected operations.
- Audit events are append-only; no update/delete via API.
- AI tool args/outputs must not contain secrets or sensitive PII beyond what's necessary and explicitly authorized.

## Migrations

- Use SQLx migrations in `migrations/` with numbered, timestamped files.
- Include appropriate indexes for expected query patterns (lists filtered by tenant/user, pagination by created_at).
- Use foreign keys with appropriate on delete behavior (cascade for dependent entities within tenant scope).
