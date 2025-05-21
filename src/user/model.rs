use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicUser {
    pub email: String,
    pub(crate) password: String,
}
