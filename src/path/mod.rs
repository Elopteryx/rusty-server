mod bfs;

use bfs::find as bfs_find;
use actix_web::Responder;
use actix_web::web::Json;

pub async fn find_with_bfs() -> impl Responder {
    Json(bfs_find())
}
