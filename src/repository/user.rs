use diesel::prelude::*;
use diesel::result::OptionalExtension;
use crate::models;
use crate::models::UpdateUser;
use crate::schema::users::dsl::*;

pub fn find_all(conn: &PgConnection) -> std::result::Result<Vec<models::User>, diesel::result::Error> {
    let result = users.load::<models::User>(conn)?;
    Ok(result)
}

#[allow(dead_code)]
pub fn find_by_id(conn: &PgConnection, user_id: i64) -> std::result::Result<Option<models::User>, diesel::result::Error> {
    let result = users.find(user_id).first::<models::User>(conn).optional()?;
    Ok(result)
}

pub fn find_by_public_id(conn: &PgConnection, user_id: uuid::Uuid) -> std::result::Result<Option<models::User>, diesel::result::Error> {
    let result = users.filter(public_id.eq(user_id)).first::<models::User>(conn).optional()?;
    Ok(result)
}

pub fn insert(conn: &PgConnection, user: UpdateUser) -> std::result::Result<models::User, diesel::result::Error> {
    let result = diesel::insert_into(users).values(&user).get_result::<models::User>(conn)?;
    Ok(result)
}

pub fn update(conn: &PgConnection, user_id: uuid::Uuid, user: UpdateUser) -> std::result::Result<Option<models::User>, diesel::result::Error> {
    let result = diesel::update(users.filter(public_id.eq(user_id))).set(name.eq(user.name)).get_result::<models::User>(conn).optional()?;
    Ok(result)
}

pub fn delete(conn: &PgConnection, user_id: uuid::Uuid) -> std::result::Result<Option<models::User>, diesel::result::Error> {
    let result = diesel::delete(users.filter(public_id.eq(user_id))).get_result::<models::User>(conn).optional()?;
    Ok(result)
}