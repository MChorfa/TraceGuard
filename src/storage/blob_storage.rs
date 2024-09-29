use azure_storage_blobs::prelude::*;
use azure_storage::prelude::*;
use std::sync::Arc;

pub struct BlobStorage {
    client: Arc<ContainerClient>,
}

impl BlobStorage {
    pub async fn new(connection_string: &str, container_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = StorageAccountClient::new_connection_string(connection_string)?
            .container_client(container_name);
        Ok(Self { client: Arc::new(client) })
    }

    pub async fn upload_blob(&self, blob_name: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .blob_client(blob_name)
            .put_block_blob(data)
            .await?;
        Ok(())
    }

    pub async fn download_blob(&self, blob_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut blob = Vec::new();
        self.client
            .blob_client(blob_name)
            .get()
            .into_stream()
            .try_for_each(|chunk| {
                blob.extend_from_slice(&chunk);
                futures::future::ready(Ok(()))
            })
            .await?;
        Ok(blob)
    }
}