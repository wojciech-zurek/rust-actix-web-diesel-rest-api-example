use actix_web::{HttpRequest, Responder};

pub async fn index(_reg: HttpRequest) -> impl Responder {
    format!("welcome, index here")
}
