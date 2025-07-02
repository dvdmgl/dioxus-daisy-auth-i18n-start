use std::collections::HashSet;

use axum_login::AuthzBackend;

use crate::{backend::BackendState, shared::user::UserPermission};

#[axum::async_trait]
impl AuthzBackend for BackendState {
    type Permission = UserPermission; // Use the defined enum

    async fn get_user_permissions(
        &self,
        _user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        // For now, we don't have user-specific permissions, so we'll return an empty set.
        // If you add user-specific permissions later, you'll query them here.
        Ok(HashSet::new())
    }

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        if let Some(groups) = self.groups.get(&user.role) {
            Ok(groups.clone())
        } else {
            Ok(HashSet::new())
        }
    }

    async fn get_all_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        if let Some(groups) = self.groups.get(&user.role) {
            Ok(groups.clone())
        } else {
            Ok(HashSet::new())
        }
    }

    async fn has_perm(
        &self,
        user: &Self::User,
        perm: Self::Permission,
    ) -> Result<bool, Self::Error> {
        if let Some(groups) = self.groups.get(&user.role) {
            Ok(groups.contains(&perm))
        } else {
            Ok(false)
        }
    }
}
