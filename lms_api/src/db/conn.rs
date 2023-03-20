use rocket_sync_db_pools::{database, diesel};

#[database("pg_database")]
pub struct MyPgDatabase(diesel::PgConnection);