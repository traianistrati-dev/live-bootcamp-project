use dotenvy::dotenv;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
pub const JWT_COOKIE_NAME: &str = "jwt";

lazy_static::lazy_static! {
    pub static ref JWT_SECRET: String = env_jwt::get_token();
}

pub mod env_jwt {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";

    pub fn get_token() -> String {
        dotenv().ok(); // Load environment variables
        let secret = std_env::var(JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
        if secret.is_empty() {
            panic!("JWT_SECRET must not be empty.");
        }
        secret
    }

    use super::*;
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
    lazy_static::lazy_static! {
        pub static ref DATABASE_URL: String =
            "postgres://postgres:123zxcQWE@localhost:5432".to_string();
    }
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
