use super::response::{AxumResponse, JsonResponse};
use crate::{agents::AgentRole, supertokens::SuperTokens};
use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue},
    middleware::Next,
};
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

        if session.status.as_str() != "OK" {
            let res = JsonResponse::send(StatusCode::UNAUTHORIZED, None, Some(session.status));
            return Err(res);
        }

        match HeaderValue::from_str(&session.session.userDataInJWT.id.to_string()) {
            Ok(x_user_id) => {
                let mut new_req = req;
                new_req.headers_mut().insert("x-user-id", x_user_id);
                Ok(next.run(new_req).await)
            }
            Err(e) => {
                let res = JsonResponse::send(StatusCode::UNAUTHORIZED, None, Some(e.to_string()));
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

    pub fn get_user_uuid(header_map: &HeaderMap) -> Option<uuid::Uuid> {
        let header_user_id = match header_map.get("x-user-id") {
            Some(id) => id,
            None => return None,
        };

        let user_id_string = match header_user_id.to_str() {
            Ok(s) => s,
            Err(_) => return None,
        };

        match uuid::Uuid::parse_str(user_id_string) {
            Ok(id) => Some(id),
            Err(_) => None,
        }
    }
}
