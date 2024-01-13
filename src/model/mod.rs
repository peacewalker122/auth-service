use crate::database::{new_db_pool, DB};
pub mod error;
pub mod user;

pub use self::error::Error;

use error::Result;
use log::info;
#[derive(Debug, Clone)]
pub struct ModelManager {
    pub db: DB,
}

impl ModelManager {
    pub async fn new() -> Result<ModelManager> {
        let db = new_db_pool().await?;

        info!("connected to DB auth");

        Ok(ModelManager { db })
    }

    pub(in crate::model) fn db(&self) -> &DB {
        &self.db
    }
}
