use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use std::time::Instant;

pub struct LoggingFairing;

#[rocket::async_trait]
impl Fairing for LoggingFairing {
    fn info(&self) -> Info {
        Info {
            name: "Logging",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        let start = Instant::now();
        request.local_cache(|| start);
        tracing::info!(
            method = %request.method(),
            uri = %request.uri(),
            "Requête reçue"
        );
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let duration = request.local_cache(|| Instant::now()).elapsed();
        tracing::info!(
            method = %request.method(),
            uri = %request.uri(),
            status = %response.status().code,
            duration_ms = duration.as_millis(),
            "Réponse envoyée"
        );
    }
}

