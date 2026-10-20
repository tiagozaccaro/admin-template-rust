# Extension Guide

This template is designed to be modular and extensible while maintaining feature parity, security, and tenant isolation.

## Adding a New Resource

To add a new resource (e.g., `projects`):

1. **Add database migration** - Add table definition to migrations with tenant scoping
2. **Add contracts** - Define request/response types in `crates/contracts`
3. **Add domain logic** - Implement business rules in appropriate crate
4. **Add API endpoints** - Create routes in `apps/api` with tenant/auth/permission middleware
5. **Add UI components** - Create web and mobile UI with same capabilities (parity)
6. **Add tests** - Include unit and integration tests

## Principles

- All tenant-owned resources must enforce tenant scoping server-side
- Authorization checks happen at the API layer (not just UI)
- Maintain feature parity between web and mobile
- Keep secrets out of client bundles
- Log security-relevant actions to audit trail
- Ensure backwards compatibility and follow existing patterns
