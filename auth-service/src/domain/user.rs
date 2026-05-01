pub struct User {
    pub email: String,
    pub passwor: String,
    requires_2fa: bool,
}

impl User {
    fn new(&self, email: &str, password: &str, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}
