use async_trait::async_trait;
use openidconnect::{
    core::{
        CoreClient, CoreIdTokenClaims, CoreProviderMetadata, CoreResponseType,
        CoreTokenResponse,
    },
    reqwest::async_http_client,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, RedirectUrl, Scope,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OIDCConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub issuer_url: String,
}

pub struct OIDCProvider {
    client: CoreClient,
}

#[async_trait]
pub trait OIDCAuthentication {
    async fn new(config: OIDCConfig) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
    async fn start_auth(&self) -> (String, CsrfToken, Nonce);
    async fn complete_auth(
        &self,
        code: AuthorizationCode,
        csrf_token: CsrfToken,
        nonce: Nonce,
    ) -> Result<CoreIdTokenClaims, Box<dyn std::error::Error>>;
}

#[async_trait]
impl OIDCAuthentication for OIDCProvider {
    async fn new(config: OIDCConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(config.issuer_url)?,
            async_http_client,
        )
        .await?;

        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(config.client_id),
            Some(ClientSecret::new(config.client_secret)),
        )
        .set_redirect_uri(RedirectUrl::new(config.redirect_uri)?);

        Ok(Self { client })
    }

    async fn start_auth(&self) -> (String, CsrfToken, Nonce) {
        let (auth_url, csrf_token, nonce) = self
            .client
            .authorize_url(
                CoreResponseType::Code,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();

        (auth_url.to_string(), csrf_token, nonce)
    }

    async fn complete_auth(
        &self,
        code: AuthorizationCode,
        csrf_token: CsrfToken,
        nonce: Nonce,
    ) -> Result<CoreIdTokenClaims, Box<dyn std::error::Error>> {
        let token_response = self
            .client
            .exchange_code(code)
            .request_async(async_http_client)
            .await?;

        let id_token = token_response
            .id_token()
            .ok_or("Server did not return an ID token")?;

        let claims = id_token.claims(&self.client.id_token_verifier(), &nonce)?;

        Ok(claims.clone())
    }
}

pub fn create_oidc_providers(
    configs: Vec<(String, OIDCConfig)>,
) -> Arc<std::collections::HashMap<String, Arc<dyn OIDCAuthentication + Send + Sync>>> {
    let mut providers = std::collections::HashMap::new();

    for (name, config) in configs {
        let provider = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(OIDCProvider::new(config))
            .unwrap();
        providers.insert(name, Arc::new(provider) as Arc<dyn OIDCAuthentication + Send + Sync>);
    }

    Arc::new(providers)
}