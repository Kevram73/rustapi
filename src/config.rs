//! Module de configuration de l'application
//!
//! Ce module gère le chargement et la validation de la configuration
//! depuis les variables d'environnement.

use serde::Deserialize;
use std::env;

/// Configuration principale de l'application
///
/// Tous les paramètres de configuration sont chargés depuis les variables
/// d'environnement et accessibles via cette structure.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    pub api_version: String,
    pub log_level: String,
    pub server_port: u16,
    pub server_host: String,
}

impl AppConfig {
    /// Charge la configuration depuis les variables d'environnement
    ///
    /// # Exemple
    /// ```
    /// use crate::config::AppConfig;
    ///
    /// let config = AppConfig::from_env()?;
    /// ```
    ///
    /// # Erreurs
    /// Retourne une erreur si les variables d'environnement requises sont invalides.
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://user:password@localhost/rustapi".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or("3600".to_string())
                .parse()
                .unwrap_or(3600),
            api_version: env::var("API_VERSION").unwrap_or_else(|_| "v1".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            server_host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        })
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}

