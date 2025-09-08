use super::Principal;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::LazyLock;
use std::time::Duration;

const DEFAULT_SECRET: &str = "se12345c@ret";
static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(|| JWT::default());

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String,
    sub: String,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug)]
pub struct JwtConfig {
    pub secret: Cow<'static, str>,
    pub expiration: Duration,
    pub audience: String,
    pub issuer: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: Cow::Borrowed(DEFAULT_SECRET),
            expiration: Duration::from_secs(60 * 60 * 24),
            audience: "audience".to_string(),
            issuer: "issuer".to_string(),
        }
    }
}

pub struct JWT {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT {
    pub fn new(config: JwtConfig) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);

        let secret = config.secret.as_bytes();
        let header = Header::new(Algorithm::HS256);
        let expiration = config.expiration;
        let audience = config.audience;
        let issuer = config.issuer;
        Self {
            encode_secret: EncodingKey::from_secret(secret),
            decode_secret: DecodingKey::from_secret(secret),
            header,
            validation,
            expiration,
            audience,
            issuer,
        }
    }
}

impl super::Auth for JWT {
    async fn encode(&self, principal: &Principal) -> ApiResult<String> {
        let current_timestamp = jsonwebtoken::get_current_timestamp();
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: format!(
                "{}:{}:{}",
                principal.tenant_id,
                principal.user_type.value(),
                principal.user_id
            ),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
        };
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_secret,
        )?)
    }

    async fn decode(&self, token: &str) -> ApiResult<Principal> {
        let claims: Claims =
            jsonwebtoken::decode(token, &self.decode_secret, &self.validation)?.claims;
        let mut parts = claims.sub.splitn(3, ":");
        let principal = Principal {
            tenant_id: parts
                .next()
                .ok_or_else(|| ApiError::InvalidToken)?
                .parse()
                .map_err(|_| ApiError::InvalidToken)?,
            user_type: parts
                .next()
                .ok_or_else(|| ApiError::InvalidToken)?
                .parse()
                .map_err(|_| ApiError::InvalidToken)?,
            user_id: parts
                .next()
                .ok_or_else(|| ApiError::InvalidToken)?
                .parse()
                .map_err(|_| ApiError::InvalidToken)?,
        };
        Ok(principal)
    }
}

impl Default for JWT {
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
}

pub fn get_default_jwt() -> &'static JWT {
    &DEFAULT_JWT
}
