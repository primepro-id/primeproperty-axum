use super::{s3::S3, s3_error::S3Error};
use crate::envs::Envs;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct S3Image {
    is_cover: bool,
    path: String,
    english_label: String,
    indonesian_label: String,
    endpoint: Option<String>,
}
impl S3Image {
    fn new(key: &str) -> Self {
        let endpoint = Envs::s3_endpoint();
        let bucket = Envs::s3_bucket();
        Self {
            is_cover: false,
            path: format!("/{bucket}/{key}"),
            english_label: "".to_string(),
            indonesian_label: "".to_string(),
            endpoint: Some(endpoint),
        }
    }
    pub async fn upload(
        bytes: Bytes,
        content_type: &str,
        extension: &str,
    ) -> Result<Self, S3Error> {
        let unique_id = cuid::cuid2();
        let s3_key = format!("{}.{}", unique_id, extension);
        let byte_stream = ByteStream::from(bytes);
        let file_upload = S3::upload_file(&s3_key, byte_stream, &content_type).await;
        match file_upload {
            Ok(_) => Ok(Self::new(&s3_key)),
            Err(err) => Err(err),
        }
    }
}
