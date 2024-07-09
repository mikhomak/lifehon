use sqlx::PgPool;

pub async fn is_registration_allowed(pg_pool: &PgPool) -> bool {
    let r_allow_registration: Result<bool, _> =
        sqlx::query_scalar!("SELECT allow_registration FROM l_site_configuration")
            .fetch_one(pg_pool)
            .await;

    r_allow_registration.unwrap_or_else(|_| false)
}

pub async fn is_login_enabled(pg_pool: &PgPool) -> bool {
    let r_is_login_enabled: Result<bool, _> =
        sqlx::query_scalar!("SELECT allow_login FROM l_site_configuration")
            .fetch_one(pg_pool)
            .await;

    r_is_login_enabled.unwrap_or_else(|_| false)
}

pub async fn is_hapi_allowed(pg_pool: &PgPool) -> bool {
    let r_allow_hobby_communication: Result<bool, _> =
        sqlx::query_scalar!("SELECT allow_hobby_communication FROM l_site_configuration")
            .fetch_one(pg_pool)
            .await;

    r_allow_hobby_communication.unwrap_or_else(|_| false)
}
