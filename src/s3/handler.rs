use super::s3_image::S3Image;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
};
use axum::{extract::Multipart, http::HeaderMap, routing::post, Router};

async fn upload_images(mut multipart: Multipart) -> AxumResponse<Vec<S3Image>> {
    let mut uploaded_files: Vec<S3Image> = Vec::new();
    while let Ok(Some(field)) = multipart.next_field().await {
        // 1. EXTRACT METADATA FOR VALIDATION
        let content_type = field.content_type().unwrap_or("").to_string();
        let file_name = field.file_name().unwrap_or("").to_string().to_lowercase();

        // 2. RUN CONTENT-TYPE VALIDATION
        // Valid MIME types: image/png, image/jpeg, image/webp
        let is_valid_mime = matches!(
            content_type.as_str(),
            "image/png" | "image/jpeg" | "image/jpg" | "image/webp"
        );

        // 3. RUN FILE EXTENSION VALIDATION
        let is_valid_ext = file_name.ends_with(".png")
            || file_name.ends_with(".jpeg")
            || file_name.ends_with(".jpg")
            || file_name.ends_with(".webp");

        // Reject if either validation fails
        if !is_valid_mime && !is_valid_ext {
            return JsonResponse::send(
                400,
                None,
                Some("Invalid file format. Only PNG, JPEG, JPG, and WEBP are allowed.".to_string()),
            );
        }
        let extension = if file_name.ends_with(".png") {
            "png"
        } else if file_name.ends_with(".webp") {
            "webp"
        } else {
            "jpg"
        };
        let bytes = field.bytes().await.unwrap();
        let file_upload = S3Image::upload(bytes, &content_type, extension).await;
        match file_upload {
            Ok(image) => {
                uploaded_files.push(image);
            }
            Err(err) => {
                println!("[upload_images] ERROR {:?}", err);
                return JsonResponse::send(500, None, Some(err.to_string()));
            }
        }
    }

    JsonResponse::send(200, Some(uploaded_files), None)
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/images", post(upload_images))
}
