#[macro_use]
extern crate diesel;

mod models;
mod schema;
mod handler;
mod repository;

use actix_web::{HttpServer, App, middleware, web};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use log::info;
use handler::user;
use handler::index;

type Result = std::io::Result<()>;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = init_connection();

    let bind = "127.0.0.1:8080";
    info!("Starting server at: {}", &bind);

    HttpServer::new(
        move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .route("/", web::get().to(index::index))
                .route("/user", web::get().to(user::get_all))
                .route("/user/{id}", web::get().to(user::get_by_public_id))
                .route("/user", web::post().to(user::create))
                .route("/user/{id}", web::put().to(user::update))
                .route("/user/{id}", web::delete().to(user::delete))
        })
        .bind(bind)?
        .run().await
}

fn init_connection() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
