pub struct Envs;

impl Envs {
    const APP_ENV: &str = "APP_ENV";
    const HOST_ADDRESS: &str = "HOST_ADDRESS";
    const DATABASE_URL: &str = "DATABASE_URL";
    const SENTRY_URL: &str = "SENTRY_URL";
    const SUPERTOKENS_CONNECTION_URI: &str = "SUPERTOKENS_CONNECTION_URI";
    const SUPERTOKENS_API_KEY: &str = "SUPERTOKENS_API_KEY";
    const S3_ENDPOINT: &str = "S3_ENDPOINT";
    const S3_BUCKET: &str = "S3_BUCKET";
    const S3_ACCESS_KEY: &str = "S3_ACCESS_KEY";
    const S3_SECRET_KEY: &str = "S3_SECRET_KEY";

    pub fn app_env() -> String {
        let expect = format!("{} must be set", Self::APP_ENV);
        std::env::var(Self::HOST_ADDRESS)
            .expect(&expect)
            .to_string()
    }

    pub fn host_address() -> String {
        let expect = format!("{} must be set", Self::HOST_ADDRESS);
        std::env::var(Self::HOST_ADDRESS)
            .expect(&expect)
            .to_string()
    }

    pub fn database_url() -> String {
        let expect = format!("{} must be set", Self::DATABASE_URL);
        std::env::var(Self::DATABASE_URL)
            .expect(&expect)
            .to_string()
    }

    pub fn sentry_url() -> String {
        let expect = format!("{} must be set", Self::SENTRY_URL);
        std::env::var(Self::SENTRY_URL).expect(&expect).to_string()
    }

    pub fn supertokens_connection_uri() -> String {
        let expect = format!("{} must be set", Self::SUPERTOKENS_CONNECTION_URI);
        std::env::var(Self::SUPERTOKENS_CONNECTION_URI)
            .expect(&expect)
            .to_string()
    }

    pub fn supertokens_api_key() -> String {
        let expect = format!("{} must be set", Self::SUPERTOKENS_API_KEY);
        std::env::var(Self::SUPERTOKENS_API_KEY)
            .expect(&expect)
            .to_string()
    }

    pub fn s3_endpoint() -> String {
        let name = Self::S3_ENDPOINT;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }
    pub fn s3_bucket() -> String {
        let name = Self::S3_BUCKET;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn s3_access_key() -> String {
        let name = Self::S3_ACCESS_KEY;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn s3_secret_key() -> String {
        let name = Self::S3_SECRET_KEY;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }
}
