mod insertion;
mod merge;
mod quick;

use insertion::sort as insertion_sort;
use merge::sort as merge_sort;
use quick::sort as quick_sort;
use actix_web::web::Json;
use actix_web::Responder;

pub async fn sort_with_insertion() -> impl Responder {
    let mut nums = vec![1, 34, 5, 7, 4];
    insertion_sort(&mut nums);
    Json(nums)
}

pub async fn sort_with_merge() -> impl Responder {
    let mut nums = vec![1, 34, 5, 7, 4];
    merge_sort(&mut nums);
    Json(nums)
}

pub async fn sort_with_quick() -> impl Responder {
    let mut nums = vec![1, 34, 5, 7, 4];
    quick_sort(&mut nums);
    Json(nums)
}
