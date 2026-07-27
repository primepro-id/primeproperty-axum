use super::s3_error::S3Error;
use crate::envs::Envs;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    config::Credentials, operation::put_object::PutObjectOutput, primitives::ByteStream,
    types::ObjectCannedAcl, Client,
};

pub(super) struct S3 {}
impl S3 {
    fn get_credential() -> Credentials {
        let access_key = Envs::s3_access_key();
        let secret_key = Envs::s3_secret_key();
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
        let endpoint = Envs::s3_endpoint();
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
        let bucket = Envs::s3_bucket();
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
