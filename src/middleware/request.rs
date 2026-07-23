use super::response::{AxumResponse, JsonResponse};
use crate::{
    agents::{Agent, AgentRole},
    supertokens::SuperTokens,
};
use axum::{extract::Request, middleware::Next};
use reqwest::StatusCode;

pub struct RequestMiddleware;

impl RequestMiddleware {
    pub async fn check_session(
        req: Request,
        next: Next,
    ) -> Result<axum::http::Response<axum::body::Body>, AxumResponse<String>> {
        let header = req.headers().get("x-access-token");
        let token = match header {
            Some(t) => t.to_str().unwrap_or(""),
            None => {
                let response = JsonResponse::send(StatusCode::UNAUTHORIZED, None, None);
                return Err(response);
            }
        };

        let session = match SuperTokens::verify_session(&token).await {
            Ok(s) => s,
            Err(e) => {
                let res = JsonResponse::send(StatusCode::UNAUTHORIZED, None, Some(e.to_string()));
                return Err(res);
            }
        };

        match session.status.as_str() {
            "OK" => Ok(next.run(req).await),
            _ => {
                let res = JsonResponse::send(StatusCode::UNAUTHORIZED, None, Some(session.status));
                return Err(res);
            }
        }
    }

    pub async fn check_admin(
        req: Request,
        next: Next,
    ) -> Result<axum::http::Response<axum::body::Body>, AxumResponse<String>> {
        let header = req.headers().get("x-access-token");
        let token = match header {
            Some(t) => t.to_str().unwrap_or(""),
            None => {
                let response = JsonResponse::send(StatusCode::UNAUTHORIZED, None, None);
                return Err(response);
            }
        };

        let session = match SuperTokens::verify_session(&token).await {
            Ok(s) => s,
            Err(e) => {
                let res = JsonResponse::send(StatusCode::UNAUTHORIZED, None, Some(e.to_string()));
                return Err(res);
            }
        };

        if session.status != "OK" {
            let res = JsonResponse::send(StatusCode::UNAUTHORIZED, None, Some(session.status));
            return Err(res);
        }

        match session.session.userDataInJWT.role {
            AgentRole::Admin => Ok(next.run(req).await),
            _ => {
                let res = JsonResponse::send(StatusCode::FORBIDDEN, None, None);
                return Err(res);
            }
        }
    }
}
