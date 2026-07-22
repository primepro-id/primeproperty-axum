use super::{
    request::{
        ConsumePasswordResetTokenRequest, CreatePasswordResetTokenRequest, CreateSessionRequest,
        SigninRequest, UpdateUserPasswordRequest,
    },
    response::{
        ConsumePasswordResetTokenResponse, CreatePasswordResetTokenResponse, CreateSessionResponse,
        SigninResponse, UpdateUserResponse,
    },
};
use crate::{agents::Agent, envs::Envs};
use serde::Serialize;

pub struct SuperTokens;

impl SuperTokens {
    // async fn get(path: &str) -> Result<reqwest::Response, reqwest::Error> {
    //     let url = format!("{}{}", Envs::supertokens_connection_uri(), path);
    //     let api_key = Envs::supertokens_api_key();

    //     reqwest::Client::new()
    //         .get(url)
    //         .header("Content-Type", "application/json")
    //         .header("api-key", api_key)
    //         .send()
    //         .await
    // }

    async fn post<T: Serialize>(
        path: &str,
        payload: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", Envs::supertokens_connection_uri(), path);
        let api_key = Envs::supertokens_api_key();

        reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/json")
            .header("api-key", api_key)
            .json(&payload)
            .send()
            .await
    }

    async fn put<T: Serialize>(
        path: &str,
        payload: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", Envs::supertokens_connection_uri(), path);
        let api_key = Envs::supertokens_api_key();

        reqwest::Client::new()
            .put(&url)
            .header("Content-Type", "application/json")
            .header("api-key", api_key)
            .json(&payload)
            .send()
            .await
    }

    const SIGNIN_PATH: &str = "/recipe/signin";
    pub async fn signin(email: &str, password: &str) -> Result<SigninResponse, reqwest::Error> {
        let req = SigninRequest {
            email: email.to_string(),
            password: password.to_string(),
        };
        let res = match Self::post(Self::SIGNIN_PATH, &req).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        res.json().await
    }

    const CREATE_SESSION_PATH: &str = "/recipe/session";
    pub async fn create_session(
        supertokens_id: &str,
        agent: &Agent,
    ) -> Result<CreateSessionResponse, reqwest::Error> {
        let req = CreateSessionRequest::new(supertokens_id, agent);
        let res = match Self::post(Self::CREATE_SESSION_PATH, &req).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        res.json().await
    }

    const CREATE_PASSWORD_RESET_TOKEN_PATH: &str = "/recipe/user/password/reset/token";
    pub async fn create_password_reset_token(
        supertokens_user_id: &str,
        email: &str,
    ) -> Result<CreatePasswordResetTokenResponse, reqwest::Error> {
        let req = CreatePasswordResetTokenRequest {
            userId: supertokens_user_id.to_string(),
            email: email.to_string(),
        };
        let res = match Self::post(Self::CREATE_PASSWORD_RESET_TOKEN_PATH, &req).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        res.json().await
    }

    const CONSUME_CREATE_PASSWORD_RESET_TOKEN: &str = "/recipe/user/password/reset/token/consume";
    pub async fn consume_password_reset_token(
        token: &str,
    ) -> Result<ConsumePasswordResetTokenResponse, reqwest::Error> {
        let req = ConsumePasswordResetTokenRequest {
            token: token.to_string(),
        };
        let res = match Self::post(Self::CONSUME_CREATE_PASSWORD_RESET_TOKEN, &req).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        res.json().await
    }

    const USER_PATH: &str = "/recipe/user";
    pub async fn update_user_password(
        recipe_user_id: &str,
        password: &str,
    ) -> Result<UpdateUserResponse, reqwest::Error> {
        let req = UpdateUserPasswordRequest {
            recipeUserId: recipe_user_id.to_string(),
            password: Some(password.to_string()),
        };
        let res = match Self::put(Self::USER_PATH, &req).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        res.json().await
    }

    const SIGNUP_PATH: &str = "/recipe/signup";
    pub async fn signup(email: &str) -> Result<SigninResponse, reqwest::Error> {
        let req = SigninRequest {
            email: email.to_string(),
            password: uuid::Uuid::new_v4().to_string(),
        };
        let res = match Self::post(Self::SIGNUP_PATH, &req).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        res.json().await
    }
}
