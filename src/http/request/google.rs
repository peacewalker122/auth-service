use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
