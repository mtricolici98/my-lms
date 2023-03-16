#[macro_use]
extern crate rocket;

use db::auth::UserDB;
use api::auth::{delete_user, login_user, regiser_user, get_me, logout_user};

use std::{sync::Arc};
use surrealdb::{Datastore, Session};

use cors::*;

mod db;
mod api;
mod error;
mod prelude;
mod utils;
pub mod cors;

#[launch]
async fn rocket() -> _ {
    let ds = Arc::new(Datastore::new("memory").await.unwrap());
    let sesh = Session::for_db("my_ns", "my_db");

    let user_db = UserDB { ds, sesh };

    rocket::build()
        .mount(
            "/auth",
            routes![regiser_user, login_user, delete_user, logout_user, get_me],/*  */
         )
        .attach(CORS)
        .manage(user_db)
}
