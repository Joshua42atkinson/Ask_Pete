use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // 1. Check for Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_header) if auth_header.starts_with("Bearer ") => {
            // let token = auth_header.trim_start_matches("Bearer ");
            // TODO: Verify JWT token here using jsonwebtoken crate
            // For now, we accept any token for the "Technical Spike"
            Ok(next.run(req).await)
        }
        _ => {
            // For dev/testing, if no header, we might want to allow it or fail
            // Err(StatusCode::UNAUTHORIZED)

            // [DEV MODE] Allow requests without token for now to avoid breaking existing flows
            Ok(next.run(req).await)
        }
    }
}
