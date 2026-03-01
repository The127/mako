pub trait AuthProvider {
    fn get_auth_header(&self) -> String;
}

pub struct ApiTokenAuthProvider {
    token: String,
}

impl ApiTokenAuthProvider {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl AuthProvider for ApiTokenAuthProvider {
    fn get_auth_header(&self) -> String {
        format!("ApiToken {}", self.token)
    }
}
