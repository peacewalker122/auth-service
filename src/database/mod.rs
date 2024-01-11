mod error;

use std::{env, str::FromStr};

pub use crate::database::error::{Error, Result};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

pub type DB = Pool<Postgres>;

pub async fn new_db_pool() -> Result<DB> {
    let opts = PgConnectOptions::from_str(&env::var("DATABASE_URL").expect("DB_URL must be set!"))
        .map_err(|e| Error::FailToCreatePool(e.to_string()))?
        .log_statements(log::LevelFilter::Trace)
        .clone();

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
        .map_err(|err| Error::FailToCreatePool(err.to_string()))
}
