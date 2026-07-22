use serde::Serialize;

use crate::agents::Agent;

#[derive(Serialize)]
pub(super) struct SigninRequest {
    pub(super) email: String,
    pub(super) password: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(super) struct CreateSessionRequest {
    userId: String,
    userDataInJWT: Agent,
    userDataInDatabase: Agent,
    enableAntiCsrf: bool,
    useDynamicSigningKey: bool,
}

impl CreateSessionRequest {
    pub(super) fn new(user_id: &str, agent: &Agent) -> Self {
        Self {
            userId: user_id.to_string(),
            userDataInJWT: agent.clone(),
            userDataInDatabase: agent.clone(),
            enableAntiCsrf: false,
            useDynamicSigningKey: false,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(super) struct VerifySessionRequest {
    accessToken: String,
    enableAntiCsrf: bool,
    doAntiCsrfCheck: bool,
    checkDatabase: bool,
}

impl VerifySessionRequest {
    pub(super) fn new(access_token: &str) -> Self {
        Self {
            accessToken: access_token.to_string(),
            enableAntiCsrf: false,
            doAntiCsrfCheck: false,
            checkDatabase: false,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(super) struct CreatePasswordResetTokenRequest {
    pub(super) userId: String,
    pub(super) email: String,
}

#[derive(Serialize)]
pub struct ConsumePasswordResetTokenRequest {
    pub(super) token: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct UpdateUserPasswordRequest {
    pub(super) recipeUserId: String,
    pub(super) password: Option<String>,
}
