use super::{request::SigninRequest, response::SigninResponse};
use crate::envs::Envs;
use serde::Serialize;

pub struct SuperTokens;

impl SuperTokens {
    async fn get(path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", Envs::supertokens_connection_uri(), path);
        let api_key = Envs::supertokens_api_key();

        reqwest::Client::new()
            .get(url)
            .header("Content-Type", "application/json")
            .header("api-key", api_key)
            .send()
            .await
    }

    async fn post<T: Serialize>(
        path: &str,
        payload: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", Envs::supertokens_connection_uri(), path);

        reqwest::Client::new()
            .post(&url)
            .json(&payload)
            .send()
            .await
    }

    const SIGNIN_PATH: &str = "/recipe/signin";
    async fn signin(email: &str) -> Result<SigninResponse, reqwest::Error> {
        let req = SigninRequest {
            email: email.to_string(),
            password: uuid::Uuid::new_v4().to_string(),
        };
        let res = match Self::post(Self::SIGNIN_PATH, &req).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        res.json().await
    }

    const SIGNUP_PATH: &str = "/recipe/signup";
    async fn signup(email: &str) -> Result<SigninResponse, reqwest::Error> {
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
