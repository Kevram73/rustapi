//! Module de gestion des erreurs
//!
//! Ce module définit le système centralisé de gestion des erreurs de l'application.
//! Toutes les erreurs sont converties automatiquement en réponses HTTP appropriées.

use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use rocket::{Request, response};
use serde_json::json;
use thiserror::Error;

/// Type d'erreur principal de l'application
///
/// Tous les types d'erreurs possibles sont définis dans cette énumération.
/// Chaque variante est automatiquement convertie en réponse HTTP avec le bon code de statut.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erreur de base de données: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Erreur de validation: {0}")]
    Validation(String),

    #[error("Erreur d'authentification: {0}")]
    Authentication(String),

    #[error("Erreur d'autorisation: {0}")]
    Authorization(String),

    #[error("Ressource non trouvée: {0}")]
    NotFound(String),

    #[error("Erreur interne du serveur: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("Requête invalide: {0}")]
    BadRequest(String),

    #[error("Erreur de sérialisation: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Erreur de base de données: {}", e);
                (Status::InternalServerError, self.to_string())
            }
            AppError::Validation(ref msg) => (Status::BadRequest, msg.clone()),
            AppError::Authentication(ref msg) => (Status::Unauthorized, msg.clone()),
            AppError::Authorization(ref msg) => (Status::Forbidden, msg.clone()),
            AppError::NotFound(ref msg) => (Status::NotFound, msg.clone()),
            AppError::Internal(ref e) => {
                tracing::error!("Erreur interne: {}", e);
                (Status::InternalServerError, "Une erreur interne s'est produite".to_string())
            }
            AppError::BadRequest(ref msg) => (Status::BadRequest, msg.clone()),
            AppError::Serialization(ref e) => {
                tracing::error!("Erreur de sérialisation: {}", e);
                (Status::BadRequest, "Erreur de format de données".to_string())
            }
        };

        let body = json!({
            "error": error_message,
            "status": status.code,
        });

        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.to_string().len(), std::io::Cursor::new(body.to_string()))
            .ok()
    }
}

pub type AppResult<T> = Result<T, AppError>;

// Catchers pour les erreurs Rocket
#[catch(404)]
pub fn not_found(_req: &Request) -> Json<serde_json::Value> {
    Json(json!({
        "error": "Route non trouvée",
        "status": 404,
    }))
}

#[catch(500)]
pub fn internal_error(_req: &Request) -> Json<serde_json::Value> {
    Json(json!({
        "error": "Une erreur interne s'est produite",
        "status": 500,
    }))
}

#[catch(400)]
pub fn bad_request(_req: &Request) -> Json<serde_json::Value> {
    Json(json!({
        "error": "Requête invalide",
        "status": 400,
    }))
}

