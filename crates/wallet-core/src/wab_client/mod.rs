// WAB client stubs within wallet-core namespace

#[derive(Debug, Default)]
pub struct WABClient;

pub mod auth_method_interactors {
    #[derive(Debug, Default)]
    pub struct TwilioPhoneInteractor;
    #[derive(Debug, Default)]
    pub struct PersonaIDInteractor;
    #[derive(Debug, Default)]
    pub struct AuthMethodInteractor; // placeholder marker type; real trait in wallet-wab-client
}
