use serde::Serialize;

#[derive(Serialize)]
pub(super) struct SigninRequest {
    pub(super) email: String,
    pub(super) password: String,
}
