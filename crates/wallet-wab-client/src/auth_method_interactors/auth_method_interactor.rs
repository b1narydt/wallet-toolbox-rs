#[derive(Debug, Default, Clone)]
pub struct AuthPayload {
    // TODO: match TS shape; e.g. { phoneNumber, otp } for Twilio, etc.
}

#[derive(Debug, Default, Clone)]
pub struct StartAuthResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct CompleteAuthResponse {
    pub success: bool,
    pub message: Option<String>,
    pub presentation_key: Option<String>,
}

pub trait AuthMethodInteractor {
    fn method_type(&self) -> &str;
    fn start_auth(&self, _server_url: &str, _presentation_key: &str, _payload: &AuthPayload) -> StartAuthResponse {
        StartAuthResponse { success: false, message: Some("not implemented".into()) }
    }
    fn complete_auth(&self, _server_url: &str, _presentation_key: &str, _payload: &AuthPayload) -> CompleteAuthResponse {
        CompleteAuthResponse { success: false, message: Some("not implemented".into()), presentation_key: None }
    }
}
