use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use time::{Duration, OffsetDateTime};

const JWT_SECRET: &[u8] = b"SUPER_SECRET_KEY_CHANGE_ME";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub typ: String, //access | refresh
}

pub fn generate_access_token(user_id: &str, role: &str) -> String {
    let exp = (OffsetDateTime::now_utc() + Duration::minutes(5)).unix_timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp,
        typ: "access".into(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap()
}

pub fn generate_refresh_token(user_id: &str, role: &str) -> String {
    let exp = (OffsetDateTime::now_utc() + Duration::days(7)).unix_timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp,
        typ: "refresh".into(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap()
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
