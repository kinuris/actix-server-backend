#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthClaims {
    pub id: String,
    pub admin: bool,
    pub exp: usize,
}
