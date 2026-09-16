use crate::util::jwt_service::{Jwt, TokenService, AuthConfig};
use actix_web::{HttpRequest, HttpResponse};
use crate::models::user::Role;

async fn get_jwt_from_token(token: String) -> Option<Jwt> {
    let auth_conf = AuthConfig::get_conf();
    let token_service = TokenService::new(&auth_conf);

    let jwt: Jwt = match token_service.validate_access_token(token) {
        Ok(jwt) => jwt,
        Err(e) => {
            println!("access token error: {e}");
            return None;
        },
    };

    Some(jwt)
}

async fn get_token_from_header(req: HttpRequest) -> Option<String> {
    let token = match req.headers().get("Authorization") {
        Some(header_value) => {
            match header_value.to_str() {
                Ok(auth_str) => {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        token
                    } else {
                        return None;
                    }
                }
                Err(_) => return None,
            }
        }
        None => return None,
    };

    println!("token: {}", token.to_string());
    return Some(token.to_string());
}

pub async fn get_jwt(req: HttpRequest, roles: Vec<Role>) -> Result<Jwt, HttpResponse> {
    let token: String = match get_token_from_header(req).await {
        Some(token) => token,
        None => return Err(HttpResponse::Forbidden().finish()),
    };

    let jwt: Jwt = match get_jwt_from_token(token).await {
        Some(jwt) => jwt,
        None => return Err(HttpResponse::Forbidden().finish()),
    };

    if !roles.contains(&jwt.role) { return Err(HttpResponse::Forbidden().finish())}

    Ok(jwt)

}
