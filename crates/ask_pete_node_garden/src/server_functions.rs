#[cfg(feature = "ssr")]
#[server(SaveReflection, "/api")]
pub async fn save_reflection(
    user_id: i64,
    challenge_name: String,
    reflection_text: String,
) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use pete_core::reflection::save_reflection_entry;
    use sqlx::PgPool;

    let pool = extract::<axum::Extension<PgPool>>().await?.0;

    save_reflection_entry(&pool, user_id, &challenge_name, &reflection_text)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
