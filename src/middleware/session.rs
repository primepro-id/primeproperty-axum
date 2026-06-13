use super::{axum_response::AxumResponse, JsonResponse};
use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue, Method},
    middleware::Next,
    response::Response,
};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct UserDataInJwt {
    id: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct VerifySessionData {
    userDataInJWT: UserDataInJwt,
}

#[derive(Deserialize, Debug)]
struct VerifySessionResponse {
    status: String,
    session: VerifySessionData,
}

pub struct Session;

impl Session {
    fn create_verify_session_payload(access_token: &str) -> serde_json::Value {
        serde_json::json!({
            "accessToken": access_token,
            "enableAntiCsrf": false,
            "doAntiCsrfCheck": false,
            "checkDatabase": true
        })
    }

    async fn verify_session(access_token: &str) -> Result<VerifySessionResponse, reqwest::Error> {
        let connection_uri =
            env::var("SUPERTOKENS_CONNECTION_URI").expect("Missing SUPERTOKENS_CONNECTION_URI");
        let supertokens_api_key =
            env::var("SUPERTOKENS_API_KEY").expect("Missing SUPERTOKENS_API_KEY");
        let url = format!("{}{}", connection_uri, "/recipe/session/verify");

        let payload = Self::create_verify_session_payload(access_token);

        reqwest::Client::new()
            .post(url)
            .header("Authorization", &supertokens_api_key)
            .json(&payload)
            .send()
            .await?
            .json()
            .await
    }

    pub fn extract_session_user_id(header_map: &HeaderMap) -> uuid::Uuid {
        let user_id = header_map
            .get("x-user-id")
            .expect("Missing x-user-id")
            .to_str()
            .unwrap_or("");
        uuid::Uuid::parse_str(user_id).expect("Invalid x-user-id")
    }

    async fn check_session(
        req: Request,
        next: Next,
    ) -> Result<axum::http::Response<axum::body::Body>, AxumResponse<String>> {
        let authorization_header = req.headers().get("x-access-token");
        let access_token = match authorization_header {
            Some(header_value) => header_value.to_str().unwrap_or(""),
            None => {
                let response = JsonResponse::send(401, None, None);
                return Err(response);
            }
        };
        let session_verification = Self::verify_session(access_token).await;

        match session_verification {
            Ok(session) if session.status == "OK" => {
                let mut new_req = req;
                let x_user_id = HeaderValue::from_str(&session.session.userDataInJWT.id)
                    .expect("Fail to convert x-user-id");
                new_req.headers_mut().insert("x-user-id", x_user_id);
                Ok(next.run(new_req).await)
            }
            _ => {
                let response = JsonResponse::send(401, None, None);
                Err(response)
            }
        }
    }

    pub async fn middleware(req: Request, next: Next) -> Result<Response, AxumResponse<String>> {
        let method = req.method();
        let path = req.uri().path();

        match method {
            &Method::GET => {
                if path == "/agents" || path == "/leads" {
                    return Self::check_session(req, next).await;
                }
                if path == "/properties" {
                    let authorization_header = req.headers().get("x-access-token");
                    match authorization_header {
                        Some(_) => return Self::check_session(req, next).await,
                        None => return Ok(next.run(req).await),
                    }
                }

                Ok(next.run(req).await)
            }
            &Method::POST if path == "/leads" => return Ok(next.run(req).await),
            _ => Self::check_session(req, next).await,
        }
    }
}
