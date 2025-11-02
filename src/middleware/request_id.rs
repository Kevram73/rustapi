use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use uuid::Uuid;

pub struct RequestIdFairing;

#[rocket::async_trait]
impl Fairing for RequestIdFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        let request_id = Uuid::new_v4().to_string();
        request.local_cache(|| request_id);
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let request_id = request.local_cache(|| Uuid::new_v4().to_string());
        response.set_header(rocket::http::Header::new("x-request-id", request_id.clone()));
    }
}

