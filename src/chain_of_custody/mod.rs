use sigstore::rekor::RekorClient;
use sigstore::tuf::Client as TufClient;
use sigstore::cosign::SignerVerifier;
use crate::error::AppError;

pub struct ChainOfCustody {
    rekor_client: RekorClient,
    tuf_client: TufClient,
    signer_verifier: SignerVerifier,
}

impl ChainOfCustody {
    pub async fn new() -> Result<Self, AppError> {
        let rekor_client = RekorClient::new("https://rekor.sigstore.dev")?;
        let tuf_client = TufClient::new("https://tuf-repo-cdn.sigstore.dev")?;
        let signer_verifier = SignerVerifier::new()?;

        Ok(Self {
            rekor_client,
            tuf_client,
            signer_verifier,
        })
    }

    pub async fn sign_and_record(&self, artifact: &[u8], key: &[u8]) -> Result<String, AppError> {
        let signature = self.signer_verifier.sign(artifact, key)?;
        let entry = self.rekor_client.create_entry(artifact, &signature).await?;
        Ok(entry.uuid)
    }

    pub async fn verify(&self, artifact: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, AppError> {
        let is_valid = self.signer_verifier.verify(artifact, signature, public_key)?;
        Ok(is_valid)
    }
}