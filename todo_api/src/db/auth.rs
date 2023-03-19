use std::{io::ErrorKind};


use diesel::{prelude::*};


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::schema::users;



#[derive(Debug, Serialize, Deserialize, Default, Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>, 
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>, 
}
 
 
pub fn add_user(new_user: InsertableUser, db_conn: &mut PgConnection) -> Result<User, std::io::Error> {
    let result = diesel::insert_into(users::table)
    .values(&new_user)
    .get_result::<User>(db_conn)
    .map_err(|_| {std::io::Error::new(ErrorKind::InvalidData, "Could not insert user")});
    Ok(User::default())
}


pub async fn get_user_by_id(id: String) -> Result<User, std::io::Error> {
    let sql = "SELECT * FROM $th";
    let tid = format!("{}", id);
    Ok(User::default())
}


pub async fn get_user_by_email( email: String) -> Result<User, std::io::Error> {
    let sql = "SELECT * FROM users WHERE email = $email";
    Ok(User::default())

}


pub async fn delete_user( id: String) -> Result<bool, crate::error::Error> {
    let sql = "Delete $th";
    let tid = format!("{}", id);
    Ok(true)
}


