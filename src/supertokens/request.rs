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
pub(super) struct CreatePasswordResetTokenRequest {
    pub(super) userId: String,
    pub(super) email: String,
}
