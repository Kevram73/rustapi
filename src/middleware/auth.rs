use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::errors::{AppError, AppResult};
use crate::config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

// Request Guard pour l'authentification
pub struct AuthenticatedUser {
    pub user_id: String,
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AppError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Récupérer le header Authorization
        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => return Outcome::Error((rocket::http::Status::Unauthorized, AppError::Authentication("Token manquant".to_string()))),
        };

        if !auth_header.starts_with("Bearer ") {
            return Outcome::Error((rocket::http::Status::Unauthorized, AppError::Authentication("Format de token invalide".to_string())));
        }

        let token = auth_header.trim_start_matches("Bearer ");

        // Récupérer la configuration depuis l'état Rocket
        let config = match request.guard::<&State<AppConfig>>().await {
            Outcome::Success(config) => config,
            _ => return Outcome::Error((rocket::http::Status::InternalServerError, AppError::Internal(anyhow::anyhow!("Configuration non disponible")))),
        };

        let decoding_key = DecodingKey::from_secret(config.jwt_secret().as_ref());
        let validation = Validation::new(Algorithm::HS256);

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                Outcome::Success(AuthenticatedUser {
                    user_id: token_data.claims.sub.clone(),
                    claims: token_data.claims,
                })
            }
            Err(_) => Outcome::Error((rocket::http::Status::Unauthorized, AppError::Authentication("Token invalide ou expiré".to_string()))),
        }
    }
}

pub fn generate_token(user_id: &str, secret: &str, expiration: u64) -> AppResult<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let now = chrono::Utc::now().timestamp() as usize;
    let exp = now + expiration as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Erreur lors de la génération du token: {}", e)))?;

    Ok(token)
}

