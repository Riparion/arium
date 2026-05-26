//! The one place arium's two authorization axes compose.
//!
//! Per-resource roles ([`crate::authz`], from `arium-authz`) and global RBAC
//! (flat permission tokens, from [`crate::auth`]) are deliberately blind to
//! each other. This bridge lives in the engine crate because it reads *both* —
//! the resource role via [`require_resource`] and the global token set via
//! [`crate::auth::list_permissions_for_user`]. `arium-authz` itself stays free
//! of any authn dependency.

use crate::authz::{require_resource, ResourceAuthority, ResourceAuthzError, ResourceRef};
use crate::pool::Pool;
use crate::wire::ResourceRole;

/// Which axis authorized a [`require_resource_or_permission`] call.
///
/// Worth surfacing (e.g. in an audit row): a grant via [`Self::GlobalPermission`]
/// is an app-wide capability reaching *into* resource scope — a support agent
/// editing a board they don't belong to — and usually deserves louder logging
/// than the ordinary [`Self::Resource`] path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceGrant {
    /// The user met the per-resource bar (holds a role `>= min_role`).
    Resource,
    /// Their resource role was absent or insufficient, but they hold the global
    /// permission token — the app-wide escape hatch over resource scope.
    GlobalPermission,
}

/// Authorize on **either** axis: a sufficient per-resource role, **or** a global
/// permission token. The one place the two authorization stories compose.
///
/// arium's two axes are deliberately blind to each other ([`require_resource`]
/// never reads `User.permissions`; the global RBAC path never reads resource
/// state), which keeps each boundary simple but means *neither alone* answers
/// "can this user act?" when an app wants a global escape hatch — a super-admin
/// or support role that can touch resources they don't belong to. Rather than
/// have every call site re-derive "owner OR super-admin" (where the two drift),
/// express it once here.
///
/// Order and semantics: the resource check runs first; only on
/// [`ResourceAuthzError::Forbidden`] does it fall back to the global token set
/// ([`crate::auth::list_permissions_for_user`], which unions direct and
/// role-derived tokens). Default-deny is preserved — an absent role *and* a
/// missing token is [`ResourceAuthzError::Forbidden`] — and a storage failure on
/// **either** lookup surfaces as [`ResourceAuthzError::Lookup`], never a silent
/// deny. The return value names which axis let the caller through.
pub async fn require_resource_or_permission(
    authority: &dyn ResourceAuthority,
    db: &Pool,
    user_id: i64,
    resource: ResourceRef<'_>,
    min_role: ResourceRole,
    permission: &str,
) -> Result<ResourceGrant, ResourceAuthzError> {
    match require_resource(authority, db, user_id, resource, min_role).await {
        Ok(_) => Ok(ResourceGrant::Resource),
        Err(ResourceAuthzError::Forbidden) => {
            // Resource axis said no — consult the global axis. A failure reading
            // tokens is still a Lookup error, not a silent deny.
            let perms = crate::auth::list_permissions_for_user(db, user_id)
                .await
                .map_err(ResourceAuthzError::Lookup)?;
            if perms.iter().any(|p| p == permission) {
                Ok(ResourceGrant::GlobalPermission)
            } else {
                Err(ResourceAuthzError::Forbidden)
            }
        }
        Err(e @ ResourceAuthzError::Lookup(_)) => Err(e),
    }
}
