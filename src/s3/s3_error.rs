/// S3Error provides a From<T: ProvideErrorMetadata> impl to extract
/// client-specific error details. This serves as a consistent backup to handling
/// specific service errors, depending on what is needed by the scenario.
/// It is used throughout the code examples for the AWS SDK for Rust.
#[derive(Debug)]
pub struct S3Error(String);
impl S3Error {
    // pub fn new(value: impl Into<String>) -> Self {
    //     S3Error(value.into())
    // }

    // pub fn add_message(self, message: impl Into<String>) -> Self {
    //     S3Error(format!("{}: {}", message.into(), self.0))
    // }
}

impl<T: aws_sdk_s3::error::ProvideErrorMetadata> From<T> for S3Error {
    fn from(value: T) -> Self {
        S3Error(format!(
            "{}: {}",
            value
                .code()
                .map(String::from)
                .unwrap_or("unknown code".into()),
            value
                .message()
                .map(String::from)
                .unwrap_or("missing reason".into()),
        ))
    }
}

impl std::error::Error for S3Error {}

impl std::fmt::Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
