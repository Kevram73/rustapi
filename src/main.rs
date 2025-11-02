mod api;
mod config;
mod database;
mod errors;
mod middleware;
mod models;
mod routes;
mod utils;

use config::AppConfig;
use database::Database;
use middleware::{cors, logging, request_id};
use rocket::fairing::AdHoc;
use crate::api::{handlers, task_handlers};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Charger les variables d'environnement depuis .env si disponible
    // En production, utilisez des variables d'environnement système
    if let Ok(path) = std::env::var("ENV_FILE") {
        load_env_file(&path).unwrap_or_else(|e| {
            eprintln!("Erreur lors du chargement du fichier .env: {}", e);
        });
    } else if std::path::Path::new(".env").exists() {
        load_env_file(".env").unwrap_or_else(|e| {
            eprintln!("Erreur lors du chargement du fichier .env: {}", e);
        });
    }

    // Initialiser le logger
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustapi=debug,rocket=info".into()),
        )
        .init();

    // Charger la configuration
    let config = AppConfig::from_env().expect("Impossible de charger la configuration");
    tracing::info!("Configuration chargée: {:?}", config);

    // Cloner les valeurs nécessaires pour AdHoc
    let database_url = config.database_url.clone();
    let server_port = config.server_port;
    let server_host = config.server_host.clone();
    let config_clone = config.clone();

    // Lancer Rocket
    let _rocket = rocket::build()
        .configure(rocket::Config::figment()
            .merge(("port", server_port))
            .merge(("address", server_host.parse::<std::net::IpAddr>().unwrap())))
        .attach(AdHoc::on_ignite("Database Setup", move |rocket| {
            let database_url = database_url.clone();
            let config = config_clone.clone();
            async move {
                // Initialiser la base de données
                let db = Database::new(&database_url)
                    .await
                    .expect("Impossible de se connecter à la base de données");
                tracing::info!("Connexion à la base de données établie");
                
                // Exécuter les migrations
                db.run_migrations().await.expect("Impossible d'exécuter les migrations");
                tracing::info!("Migrations exécutées");

                rocket
                    .manage(db)
                    .manage(config)
                    .attach(cors::CorsFairing)
                    .attach(logging::LoggingFairing)
                    .attach(request_id::RequestIdFairing)
            }
        }))
        .mount("/api", routes::get_routes())
        .launch()
        .await?;

    Ok(())
}

fn load_env_file(path: &str) -> anyhow::Result<()> {
    let contents = std::fs::read_to_string(path)?;
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            std::env::set_var(key, value);
        }
    }
    Ok(())
}

