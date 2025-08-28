use super::auth_method_interactor::{AuthMethodInteractor, AuthPayload, StartAuthResponse, CompleteAuthResponse};

#[derive(Debug, Default, Clone)]
pub struct TwilioPhoneInteractor;

impl AuthMethodInteractor for TwilioPhoneInteractor {
    fn method_type(&self) -> &str { "TwilioPhone" }
    fn start_auth(&self, _server_url: &str, _presentation_key: &str, _payload: &AuthPayload) -> StartAuthResponse {
        StartAuthResponse { success: false, message: Some("not implemented".into()) }
    }
    fn complete_auth(&self, _server_url: &str, _presentation_key: &str, _payload: &AuthPayload) -> CompleteAuthResponse {
        CompleteAuthResponse { success: false, message: Some("not implemented".into()), presentation_key: None }
    }
}
