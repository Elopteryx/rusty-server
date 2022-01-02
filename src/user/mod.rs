extern crate rand;

use rand::Rng;
use actix_web::web::Json;
use actix_web::Responder;

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    balance: u32,
    language: String,
}

pub async fn user_current() -> impl Responder {
    let mut rng = rand::thread_rng();
    Json(User {
        name: String::from("Anonymous"),
        balance: rng.gen_range(1000..1500),
        language: String::from("en"),
    })
}
