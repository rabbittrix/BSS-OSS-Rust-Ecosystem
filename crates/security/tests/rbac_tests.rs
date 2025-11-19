//! Unit tests for RBAC

#[cfg(test)]
mod tests {
    use security::models::Permission;
    use security::rbac::RbacService;
    use test_utils::database::create_test_pool;
    use uuid::Uuid;

    async fn setup() -> RbacService {
        let pool = create_test_pool()
            .await
            .expect("Failed to create test pool");
        RbacService::new(pool)
    }

    #[tokio::test]
    async fn test_create_role() {
        let rbac = setup().await;

        let permissions = vec![
            Permission::new("catalog".to_string(), "read".to_string()),
            Permission::new("catalog".to_string(), "write".to_string()),
        ];

        let role = rbac
            .create_role(
                "admin".to_string(),
                Some("Administrator role".to_string()),
                permissions.clone(),
            )
            .await
            .expect("Failed to create role");

        assert_eq!(role.name, "admin");
        assert_eq!(role.permissions.len(), 2);
    }

    #[tokio::test]
    async fn test_assign_role() {
        let rbac = setup().await;
        let identity_id = Uuid::new_v4();

        let permissions = vec![Permission::new("catalog".to_string(), "read".to_string())];
        let role = rbac
            .create_role("viewer".to_string(), None, permissions)
            .await
            .expect("Failed to create role");

        let user_role = rbac
            .assign_role(identity_id, role.id, None, None)
            .await
            .expect("Failed to assign role");

        assert_eq!(user_role.identity_id, identity_id);
        assert_eq!(user_role.role_id, role.id);
    }

    #[tokio::test]
    async fn test_has_permission() {
        let rbac = setup().await;
        let identity_id = Uuid::new_v4();

        let permissions = vec![Permission::new("catalog".to_string(), "read".to_string())];
        let role = rbac
            .create_role("viewer".to_string(), None, permissions)
            .await
            .expect("Failed to create role");

        rbac.assign_role(identity_id, role.id, None, None)
            .await
            .expect("Failed to assign role");

        let has_permission = rbac
            .has_permission(identity_id, "catalog", "read")
            .await
            .expect("Failed to check permission");

        assert!(has_permission);
    }

    #[tokio::test]
    async fn test_get_identity_roles() {
        let rbac = setup().await;
        let identity_id = Uuid::new_v4();

        let permissions = vec![Permission::new("catalog".to_string(), "read".to_string())];
        let role = rbac
            .create_role("viewer".to_string(), None, permissions)
            .await
            .expect("Failed to create role");

        rbac.assign_role(identity_id, role.id, None, None)
            .await
            .expect("Failed to assign role");

        let roles = rbac
            .get_identity_roles(identity_id)
            .await
            .expect("Failed to get identity roles");

        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].name, "viewer");
    }
}
