use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    config::Credentials, operation::put_object::PutObjectOutput, primitives::ByteStream,
    types::ObjectCannedAcl, Client,
};

use super::s3_error::S3Error;

pub(super) struct S3 {}
impl S3 {
    fn get_credential() -> Credentials {
        let access_key = std::env::var("S3_ACCESS_KEY").expect("Missing S3_ACCESS_KEY");
        let secret_key = std::env::var("S3_SECRET_KEY").expect("Missing S3_SECRET_KEY");
        Credentials::new(
            access_key,
            secret_key,
            None, // Session token (optional)
            None, // Expiry (optional)
            "biznetgio",
        )
    }

    async fn get_client() -> Client {
        let credential = Self::get_credential();
        let endpoint = std::env::var("S3_ENDPOINT").expect("Missing S3_ENDPOINT");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("idn"))
            .endpoint_url(endpoint)
            .credentials_provider(credential)
            .load()
            .await;
        Client::new(&config)
    }

    pub(super) async fn upload_file(
        s3_key: &str,
        body: ByteStream,
        content_type: &str,
    ) -> Result<PutObjectOutput, S3Error> {
        let bucket = std::env::var("S3_BUCKET").expect("Missing S3_BUCKET");
        let client = Self::get_client().await;
        client
            .put_object()
            .bucket(bucket)
            .key(s3_key)
            .body(body)
            .content_type(content_type)
            .acl(ObjectCannedAcl::PublicRead)
            .send()
            .await
            .map_err(S3Error::from)
    }
}
