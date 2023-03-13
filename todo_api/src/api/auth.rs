#[macro_use]

use rocket::{serde::json::Json, State, Route, post, get, delete, routes};

use std::{io::ErrorKind};
use surrealdb::{sql::Object};

use crate::{db::auth::{UserDB, AffectedRows, User}};


#[post("/register", format= "json", data="<user>")]
pub async fn regiser_user(user: Json<User>, db: &State<UserDB>) -> Result<Json<Object>, std::io::Error> {
    let task = db
        .add_user("something".into(), "something_else".into())
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to create task."))?;

    Ok(Json(task))
}

#[get("/login")]
pub async fn login_user(db: &State<UserDB>) -> Result<Json<Vec<Object>>, std::io::Error> {
    let tasks = db
        .get_all_tasks()
        .await
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to fetch all tasks."))?;

    Ok(Json(tasks))
}


#[delete("/delete", format="json", data="<user>")]
pub async fn delete_user(user: Json<User>, db: &State<UserDB>) -> Result<Json<AffectedRows>, std::io::Error> {
    match user.id.clone() {
        Some(id) => {
            let affected_rows = db
            .delete_user(id)
            .await
            .map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to delete task."))?;
            Ok(Json(affected_rows))
        },
        None => Ok(Json(AffectedRows { rows_affected: 0 }))
    }
}


// #[get("/auth/logout")]
// async fn logout_user(db: &State<DB>) -> Result<Json<String>, std::io::Error> {
//     db.logout().await.map_err(|_| std::io::Error::new(ErrorKind::Other, "Unable to logout."))
// }
