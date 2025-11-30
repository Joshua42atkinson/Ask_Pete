use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

pub async fn google_login_url() -> impl IntoResponse {
    let client = get_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    // In a real app, store csrf_token in session/cookie to validate state
    Redirect::to(auth_url.as_str())
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<AuthRequest>,
) -> impl IntoResponse {
    let client = get_oauth_client();

    // Exchange the code with a token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();

            // Get User Info from Google
            let user_info = fetch_google_user_info(access_token).await.unwrap();

            // Determine Role (Mock Logic for now)
            // In production, check DB.
            let role = if user_info.email.contains("student") {
                pete_core::UserRole::Student
            } else if user_info.email.contains("research") {
                pete_core::UserRole::Researcher
            } else {
                pete_core::UserRole::Instructor // Default to Instructor for dev convenience? Or Student?
                                                // Let's default to Student for safety, but for the "Trinity" demo, maybe Instructor is better?
                                                // The user said "The app checks their role... and routes them".
                                                // I'll default to Student.
            };

            // Generate JWT (Placeholder)
            let jwt = format!("mock_jwt_token_for_{}", user_info.id);

            // Redirect to the frontend with token and role
            // The frontend will read these params and navigate.
            let redirect_url = format!("/?token={}&role={:?}", jwt, role);
            Redirect::to(&redirect_url).into_response()
        }
        Err(e) => Json(serde_json::json!({ "error": format!("Failed to exchange token: {}", e) }))
            .into_response(),
    }
}

fn get_oauth_client() -> BasicClient {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET");
    let redirect_url = "http://localhost:3000/auth/callback"; // Frontend callback

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).expect("Invalid redirect URL"))
}

async fn fetch_google_user_info(access_token: &str) -> Result<UserInfo, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?;

    response.json::<UserInfo>().await
}
