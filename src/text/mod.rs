mod overlap;

use overlap::reassemble;
use actix_web::Responder;
use actix_web::web::Json;

pub async fn text_overlap_short() -> impl Responder {
    let line = "O draconia;conian devil! Oh la;h lame sa;saint!".to_string();
    let result = reassemble(&line);
    Json(result)
}

pub async fn text_overlap_long() -> impl Responder {
    let line = "m quaerat voluptatem.;pora incidunt ut labore et d;, consectetur, adipisci velit;olore magnam aliqua;idunt ut labore et dolore magn;uptatem.;i dolorem ipsum qu;iquam quaerat vol;psum quia dolor sit amet, consectetur, a;ia dolor sit amet, conse;squam est, qui do;Neque porro quisquam est, qu;aerat voluptatem.;m eius modi tem;Neque porro qui;, sed quia non numquam ei;lorem ipsum quia dolor sit amet;ctetur, adipisci velit, sed quia non numq;unt ut labore et dolore magnam aliquam qu;dipisci velit, sed quia non numqua;us modi tempora incid;Neque porro quisquam est, qui dolorem i;uam eius modi tem;pora inc;am al".to_string();
    let result = reassemble(&line);
    Json(result)
}
