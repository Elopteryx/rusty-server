use actix_web::web::Json;
use actix_web::Responder;

#[derive(Serialize)]
struct Version {
    value: String,
    major: i32,
    minor: i32,
    patch: i32,
}

pub async fn version() -> impl Responder {
    Json(Version {
        value: String::from("1.0.0"),
        major: 1,
        minor: 0,
        patch: 0,
    })
}
