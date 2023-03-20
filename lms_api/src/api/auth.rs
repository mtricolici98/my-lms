#[macro_use]

use rocket::{serde::json::Json, post, get, delete};
use rocket::{http::{CookieJar, Cookie}};



use crate::{db::{conn::MyPgDatabase, auth::{InsertableUser}}, utils::hashing::hash_passwd};
use crate::db::auth::User;
use crate::db::auth::add_user;

use super::view_models::{RegistrationUser, LoginUser};


#[post("/register", format= "json", data="<user>")]
pub async fn regiser_user(user: Json<RegistrationUser>, db: MyPgDatabase, cookies: &CookieJar<'_>) -> Result<Json<User>, std::io::Error> {
    let pass_hash = hash_passwd(user.pass_text.clone());
    let new_user = InsertableUser {
        id: uuid::Uuid::new_v4(),
        username: user.username.clone(),
        email: user.email.clone(),
        password_hash: Some(pass_hash),
    };
    let user = db.run(|c| add_user(
        new_user,
        c
    )).await?;
    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    Ok(Json(user))
}

#[get("/login", format= "json", data="<login_user>")]
pub async fn login_user(login_user: Json<LoginUser>, db: MyPgDatabase, cookies: &CookieJar<'_>) -> Result<Json<User>, std::io::Error> {
    // let user = db.run(|c| get_user_by_email(login_user.email.clone())).await?;
    Ok(Json(User::default()))
    // let login_pass_hash = hash_passwd(login_user.pass_text.clone());
    // let user_pass_hash = user.password_hash.clone().unwrap();
    // if login_pass_hash == user_pass_hash {
    //     match user.id.clone() {
    //     Some(id) => {
    //         cookies.add_private(Cookie::new("user_id", id.to_string()));
    //         Ok(Json(user))
    //     },
    //     None => Err(std::io::Error::new(ErrorKind::Other, "Unable to create ."))
    //     }
    // } else { 
    //     Err(std::io::Error::new(ErrorKind::Other, "User not found."))
    // }
}


#[get("/me")]
pub async fn get_me(current_user: User) -> Result<Json<User>, std::io::Error> {
    Ok(Json(current_user))
}


#[delete("/delete")]
pub async fn delete_user(db: MyPgDatabase, cookies: &CookieJar<'_>, current_user: User) -> Result<Json<bool>, std::io::Error> {
    // match current_user.id.clone() {
    //     Some(id) => {
    //         let affected_rows = db
    //         .delete_user(id)
    //         .await
    //         .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to delete task."))?;
    //         Ok(Json(affected_rows))
    //     },
    //     None => Ok(Json(AffectedRows { rows_affected: 0 }))
    // }
    Ok(Json(true))
}


#[get("/auth/logout")]
pub async fn logout_user(db: MyPgDatabase, user: User, cookies: &CookieJar<'_>) -> Result<Json<String>, std::io::Error> {
    cookies.remove_private(Cookie::named("user_id"));
    Ok(Json(String::from("Success")))
}
