#[macro_use]
extern crate serde_derive;

mod error;
mod user;
mod version;
mod delay;
mod path;
mod store;
mod sorting;
mod text;

use crate::error::{render_403, render_404};
use crate::user::user_current;
use crate::version::version;
use crate::delay::delay_by;
use crate::store::{item_all, item_by_id, prepare_database};
use crate::path::{find_with_bfs};
use crate::sorting::{sort_with_insertion, sort_with_merge, sort_with_quick};
use crate::text::{text_overlap_short, text_overlap_long};

use actix_web::{web, App, HttpServer};
use actix_web::web::scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match prepare_database() {
        Ok(_) => println!("Database initialized successfully!"),
        Err(e) => panic!("{}", e),
    }
    HttpServer::new(|| {
        App::new()
            .service(
                scope("/app/v1")
                    .route("/user", web::get().to(user_current))
                    .route("/version", web::get().to(version))
                    .route("/delay/{count}", web::get().to(delay_by))
                    .route("/store/items", web::get().to(item_all))
                    .route("/store/items/{id}", web::get().to(item_by_id))
                    .route("/path/bfs", web::get().to(find_with_bfs))
                    .route("/sort/insertion", web::get().to(sort_with_insertion))
                    .route("/sort/merge", web::get().to(sort_with_merge))
                    .route("/sort/quick", web::get().to(sort_with_quick))
                    .route("/text/overlap/short", web::get().to(text_overlap_short))
                    .route("/text/overlap/long", web::get().to(text_overlap_long))
                    .route("/admin", web::get().to(render_403))
            )
            .default_service(web::get().to(render_404))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}