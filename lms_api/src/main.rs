#[macro_use]
extern crate rocket;

use api::auth::{delete_user, login_user, regiser_user, get_me, logout_user};

use cors::*;

mod db;
mod api;
mod error;
mod prelude;
mod utils;
pub mod schema;
pub mod cors;

use crate::db::conn::MyPgDatabase;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/auth",
            routes![regiser_user, login_user, delete_user, logout_user, get_me],/*  */
         )
        .attach(CORS)
        .attach(MyPgDatabase::fairing())
}
