#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            requires_2fa,
        }
    }
}
