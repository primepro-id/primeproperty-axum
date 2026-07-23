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

#[derive(Debug, Serialize)]
pub struct DataAndPagination<T: Serialize> {
    data: Option<T>,
    pagination: Pagination,
}

impl<T: Serialize> DataAndPagination<T> {
    pub fn new(data: Option<T>, pagination: Pagination) -> Self {
        Self { data, pagination }
    }
}

const DEFAULT_PER_PAGE: u32 = 30;

#[derive(Debug, Serialize)]
pub struct Pagination {
    page: u32,
    per_page: u32,
    total_pages: u32,
    total: u32,
}

impl Pagination {
    pub fn new(page: Option<u32>, per_page: Option<u32>, total: u32) -> Self {
        let current_page = match page {
            Some(p) => p,
            None => 1,
        };
        let current_per_page = match per_page {
            Some(p) => p,
            None => DEFAULT_PER_PAGE,
        };

        let total_pages = (total as f32 / current_per_page as f32).ceil() as u32;
        Self {
            page: current_page,
            per_page: current_per_page,
            total_pages,
            total,
        }
    }
}
