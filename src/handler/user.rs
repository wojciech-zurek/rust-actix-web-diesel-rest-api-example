use actix_web::{HttpRequest, web, HttpResponse};
use log::{info, error};
use crate::DbPool;
use crate::repository::user;
use crate::models::UpdateUser;

pub async fn get_all(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    info!("{:?}", req);
    let con = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || user::find_all(&con))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => e
    }
}

pub async fn get_by_public_id(req: HttpRequest, id: web::Path<uuid::Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
    info!("{:?}", req);
    let con = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || user::find_by_public_id(&con, id.into_inner()))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(option) => {
            match option {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().finish()
            }
        }
        Err(e) => e
    }
}

pub async fn create(req: HttpRequest, user: web::Json<UpdateUser>, pool: web::Data<DbPool>) -> HttpResponse {
    info!("{:?}", req);
    let con = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || user::insert(&con, user.0))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(u) => {
            HttpResponse::Created().json(u)
        }
        Err(e) => e
    }
}

pub async fn update(req: HttpRequest, id: web::Path<uuid::Uuid>, user: web::Json<UpdateUser>, pool: web::Data<DbPool>) -> HttpResponse {
    info!("{:?}", req);
    let con = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || user::update(&con, id.into_inner(), user.0))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(option) => {
            match option {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().finish()
            }
        }
        Err(e) => e
    }
}

pub async fn delete(_req: HttpRequest, id: web::Path<uuid::Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
    info!("{:?}", _req);
    let con = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || user::delete(&con, id.into_inner()))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(option) => {
            match option {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().finish()
            }
        }
        Err(e) => e
    }
}