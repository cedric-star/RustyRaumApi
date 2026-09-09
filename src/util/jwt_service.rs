use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use dotenv::dotenv;
use std::env;

use crate::models::user::Role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub user: Uuid,
    pub exp_time: i64,
    pub start_time: i64,
    pub typ: String,
    pub role: Role,
}

impl Jwt {
    pub fn new(user: Uuid, role: Role, expires_in: Duration) -> Self {
        let now = Utc::now();

        Self {
            user: user,
            exp_time: (now + expires_in).timestamp(),
            start_time: now.timestamp(),
            typ: "access".to_string(),
            role: role,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp_time
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshJwt {
    pub user: Uuid,
    pub exp_time: i64,
    pub start_time: i64,
    pub typ: String,
    pub family: Uuid,
}

impl RefreshJwt {
    pub fn new(user: Uuid, expires_in: Duration) -> Self {
        let now = Utc::now();

        Self {
            user: user,
            exp_time: (now + expires_in).timestamp(),
            start_time: now.timestamp(),
            typ: "refresh".to_string(),
            family: Uuid::new_v4(),
        }
    }
}

struct AuthConfig {
    jwt_secret: String,
    access_token_ttl_minutes: i64,
    refresh_token_ttl_days: i64,
}

impl AuthConfig {
    fn get_conf() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").unwrap(),
            access_token_ttl_minutes: 60,
            refresh_token_ttl_days: 7,
        }
    }
}

struct TokenPair {
    access_token: String,
    refresh_token: String,
    token_typ: String,
    expires_in: u64,
}

pub struct TokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_ttl: chrono::Duration,
    refresh_token_ttl: chrono::Duration,
}

impl TokenService {
    pub fn new(config: &AuthConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.jwt_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            access_token_ttl: chrono::Duration::minutes(config.access_token_ttl_minutes),
            refresh_token_ttl: chrono::Duration::days(config.refresh_token_ttl_days),
        }
    }

    pub fn generate_jwt(&self, user: Uuid, role: Role) -> Result<String, String> {
        let jwt = Jwt::new(user, role, self.access_token_ttl);

        encode(&Header::new(Algorithm::HS256), &jwt, &self.encoding_key)
            .map_err(|_| "Encoding of Token failed!".to_string())
    }

    pub fn generate_refresh_jwt(&self, user: Uuid) -> Result<String, String> {
        let jwt = RefreshJwt::new(user, self.refresh_token_ttl);

        encode(&Header::new(Algorithm::HS256), &jwt, &self.encoding_key)
            .map_err(|_| "Encoding of regresh Token failed!".to_string())
    }

    pub fn generate_token_pair(&self, user: Uuid, role: Role) -> Result<TokenPair, String> {
        Ok(TokenPair {
            access_token: self.generate_jwt(user, role)?,
            refresh_token: self.generate_refresh_jwt(user)?,
            token_typ: "Bearer".to_string(),
            expires_in: self.access_token_ttl.num_seconds() as u64,
        })
    }

    pub fn validate_access_token(&self, token: String) -> Result<Jwt, String> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data: TokenData<Jwt> = decode(token.as_str(), &self.decoding_key, &validation)
            .map_err(|_| "Token Validation failed!".to_string())?;

        if token_data.claims.typ != "access" {
            return Err("Token Type is invalid!".to_string());
        }

        Ok(token_data.claims)
    }

    pub fn validate_refresh_token(&self, token: String) -> Result<RefreshJwt, String> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data: TokenData<RefreshJwt> = decode(token.as_str(), &self.decoding_key, &validation)
            .map_err(|_| "Refresh Token Validation failed!".to_string())?;

        if token_data.claims.typ != "refresh" {
            return Err("Refresh Token Type is invalid!".to_string());
        }

        Ok(token_data.claims)
    }
}
