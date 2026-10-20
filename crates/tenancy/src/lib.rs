use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TenantMembership {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub role: permissions::Role,
}
