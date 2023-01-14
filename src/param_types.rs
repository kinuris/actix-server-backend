use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthQueryParams {
    pub admin: Option<bool>,
}
