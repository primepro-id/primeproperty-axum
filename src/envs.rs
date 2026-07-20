pub struct Envs;

impl Envs {
    const APP_ENV: &str = "APP_ENV";
    const HOST_ADDRESS: &str = "HOST_ADDRESS";
    const FRONTEND_URL: &str = "FRONTEND_URL";
    const DATABASE_URL: &str = "DATABASE_URL";
    const SENTRY_URL: &str = "SENTRY_URL";
    const SUPERTOKENS_CONNECTION_URI: &str = "SUPERTOKENS_CONNECTION_URI";
    const SUPERTOKENS_API_KEY: &str = "SUPERTOKENS_API_KEY";
    const S3_ENDPOINT: &str = "S3_ENDPOINT";
    const S3_BUCKET: &str = "S3_BUCKET";
    const S3_ACCESS_KEY: &str = "S3_ACCESS_KEY";
    const S3_SECRET_KEY: &str = "S3_SECRET_KEY";
    const SMTP_HOST: &str = "SMTP_HOST";
    const SMTP_USER: &str = "SMTP_USER";
    const SMTP_PASS: &str = "SMTP_PASS";
    const SMTP_FROM: &str = "SMTP_FROM";

    pub fn app_env() -> String {
        let name = Self::APP_ENV;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn host_address() -> String {
        let name = Self::HOST_ADDRESS;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn frontend_url() -> String {
        let name = Self::FRONTEND_URL;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn database_url() -> String {
        let name = Self::DATABASE_URL;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn sentry_url() -> String {
        let expect = format!("{} must be set", Self::SENTRY_URL);
        std::env::var(Self::SENTRY_URL).expect(&expect).to_string()
    }

    pub fn supertokens_connection_uri() -> String {
        let name = Self::SUPERTOKENS_CONNECTION_URI;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn supertokens_api_key() -> String {
        let name = Self::SUPERTOKENS_API_KEY;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
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

    pub fn smtp_host() -> String {
        let name = Self::SMTP_HOST;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn smtp_user() -> String {
        let name = Self::SMTP_USER;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn smtp_pass() -> String {
        let name = Self::SMTP_PASS;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }

    pub fn smtp_from() -> String {
        let name = Self::SMTP_FROM;
        let expect = format!("{} must be set", name);
        std::env::var(name).expect(&expect).to_string()
    }
}
