pub struct LoginRequest {
    pub sso_ticket: String
}

pub enum MessageEvent {
    Login(LoginRequest)
}