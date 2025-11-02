use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub struct CorsFairing;

#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
        response.set_header(rocket::http::Header::new("Access-Control-Expose-Headers", "*"));
    }
}

