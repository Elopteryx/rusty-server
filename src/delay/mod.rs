use actix_web::Responder;
use actix_web::web::Path;
use async_std::task::sleep;
use std::time::Duration;

#[derive(Deserialize)]
pub struct Info {
    count: u64,
}

pub async fn delay_by(info: Path<Info>) -> impl Responder {
    let seconds = Duration::from_secs(info.count);
    sleep(seconds).await;
    format!("Hello, delayed by {} seconds!", &info.count)
}