use async_graphql::{Context, Error, Guard};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Role {
    User,
    Anon,
    Admin,
}

pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}
