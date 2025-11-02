//! Module de gestion de la base de données
//!
//! Ce module gère la connexion à PostgreSQL et fournit un pool de connexions
//! réutilisable pour toutes les opérations de base de données.

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

/// Gestionnaire de base de données
///
/// Encapsule le pool de connexions PostgreSQL et fournit des méthodes
/// pour accéder à la base de données et exécuter les migrations.
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    // Méthode générique pour exécuter des migrations
    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}

// Trait pour les repositories génériques
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> anyhow::Result<Option<T>>;
    async fn find_all(&self, limit: Option<u64>, offset: Option<u64>) -> anyhow::Result<Vec<T>>;
    async fn create(&self, entity: T) -> anyhow::Result<T>;
    async fn update(&self, id: ID, entity: T) -> anyhow::Result<T>;
    async fn delete(&self, id: ID) -> anyhow::Result<()>;
}

