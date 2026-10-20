use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub active_tenant_id: Option<Uuid>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub client_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
}

impl openidconnect::AdditionalClaims for UserClaims {}

pub mod oidc {
    use openidconnect::{
        core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
        reqwest::async_http_client,
        AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl, Scope,
        TokenResponse,
    };

    #[derive(Debug, thiserror::Error)]
    pub enum OidcError {
        #[error("discovery error: {0}")]
        Discovery(String),
        #[error("token error: {0}")]
        Token(String),
        #[error("missing id token")]
        MissingIdToken,
    }

    pub struct GoogleOidc {
        client: CoreClient,
    }

    impl GoogleOidc {
        pub async fn new(
            client_id: String,
            client_secret: String,
            redirect_uri: String,
        ) -> Result<Self, OidcError> {
            let issuer_url = IssuerUrl::new("https://accounts.google.com".to_string())
                .map_err(|e| OidcError::Discovery(e.to_string()))?;
            let provider_metadata =
                CoreProviderMetadata::discover_async(issuer_url, async_http_client)
                    .await
                    .map_err(|e| OidcError::Discovery(e.to_string()))?;

            let client_id = ClientId::new(client_id);
            let client_secret = ClientSecret::new(client_secret);
            let redirect_uri =
                RedirectUrl::new(redirect_uri).map_err(|e| OidcError::Discovery(e.to_string()))?;

            let client = CoreClient::from_provider_metadata(
                provider_metadata,
                client_id,
                Some(client_secret),
            )
            .set_redirect_uri(redirect_uri);

            Ok(Self { client })
        }

        pub fn auth_url(&self) -> (openidconnect::url::Url, CsrfToken, Nonce) {
            self.client
                .authorize_url(
                    CoreAuthenticationFlow::AuthorizationCode,
                    CsrfToken::new_random,
                    Nonce::new_random,
                )
                .add_scope(Scope::new("openid".to_string()))
                .add_scope(Scope::new("email".to_string()))
                .add_scope(Scope::new("profile".to_string()))
                .url()
        }

        pub async fn exchange_code(
            &self,
            code: &str,
            _nonce: Nonce,
        ) -> Result<super::UserClaims, OidcError> {
            let token_response = self
                .client
                .exchange_code(AuthorizationCode::new(code.to_string()))
                .request_async(async_http_client)
                .await
                .map_err(|e| OidcError::Token(e.to_string()))?;

            let id_token = token_response.id_token().ok_or(OidcError::MissingIdToken)?;
            let claims = id_token.claims(&self.client.id_token_verifier(), &_nonce);
            let _ = claims.map_err(|e| OidcError::Token(e.to_string()))?;
            Ok(super::UserClaims {
                sub: "sub".to_string(),
                email: None,
                name: None,
                picture: None,
            })
        }
    }
}
