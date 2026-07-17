use axum::{http::StatusCode, Json};
use serde::Serialize;
use std::usize;
use tracing::error;

pub type AxumResponse<T> = (StatusCode, Json<JsonResponse<T>>);

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    status: u16,
    data: Option<T>,
    message: String,
}

impl<T: Serialize> JsonResponse<T> {
    fn new(status: u16, data: Option<T>, message: String) -> Self {
        Self {
            status,
            data,
            message,
        }
    }

    pub fn send(status: StatusCode, data: Option<T>, message: Option<String>) -> AxumResponse<T> {
        let new_message = match message.clone() {
            Some(msg) => msg,
            None => match data {
                Some(_) => status.to_string(),
                None => "".to_string(),
            },
        };

        let appropriate_status = [200, 201];
        if !appropriate_status.contains(&status.as_u16()) {
            let error_message = match message {
                Some(msg) => msg,
                None => "Unknown error".to_string(),
            };
            error!("{:?}", error_message)
        }

        let response = Self::new(status.as_u16(), data, new_message);
        (status, Json(response))
    }
}

// #[derive(Debug, Serialize)]
// pub struct DataPagination<T: Serialize> {
//     data: Option<T>,
//     pagination: Pagination,
// }

// impl<T: Serialize> DataPagination<T> {
//     pub fn new(data: Option<T>, pagination: Pagination) -> Self {
//         Self { data, pagination }
//     }
// }
