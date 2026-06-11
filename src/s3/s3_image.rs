use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use super::{s3::S3, s3_error::S3Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct S3Image {
    is_cover: bool,
    path: String,
    english_label: String,
    indonesian_label: String,
}
impl S3Image {
    fn new(label: &str, key: &str) -> Self {
        let endpoint = std::env::var("S3_ENDPOINT").expect("Missing S3_ENDPOINT");
        let bucket = std::env::var("S3_BUCKET").expect("Missing S3_BUCKET");
        Self {
            is_cover: false,
            path: format!("{endpoint}/{bucket}/{key}"),
            english_label: label.to_string(),
            indonesian_label: label.to_string(),
        }
    }
    pub async fn upload(
        bytes: Bytes,
        content_type: &str,
        label: &str,
        extension: &str,
    ) -> Result<Self, S3Error> {
        let unique_id = cuid::cuid2();
        let s3_key = format!("{}.{}", unique_id, extension);
        let byte_stream = ByteStream::from(bytes);
        let file_upload = S3::upload_file(&s3_key, byte_stream, &content_type).await;
        match file_upload {
            Ok(_) => Ok(Self::new(&label, &s3_key)),
            Err(err) => Err(err),
        }
    }
}
