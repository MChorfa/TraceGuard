use azure_storage_blobs::prelude::*;
use azure_storage::prelude::*;
use std::sync::Arc;
use minio::s3::client::Client;
use minio::s3::creds::Credentials;
use minio::s3::types::{BucketName, ObjectName};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    pub tenant_id: Uuid,
    pub artifact_type: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub encryption_type: EncryptionType,
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EncryptionType {
    AES256,
    PostQuantum,
    Homomorphic,
}

#[async_trait]
pub trait BlobStorage {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>, metadata: Metadata) -> Result<(), StorageError>;
    async fn get_object(&self, bucket: &str, key: &str) -> Result<(Vec<u8>, Metadata), StorageError>;
    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), StorageError>;
    async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<ObjectInfo>, StorageError>;
    async fn update_metadata(&self, bucket: &str, key: &str, metadata: Metadata) -> Result<(), StorageError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectInfo {
    pub key: String,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
    pub metadata: Metadata,
}

pub struct MinioStorage {
    client: Client,
}

impl MinioStorage {
    pub fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Self {
        let creds = Credentials::new(Some(access_key), Some(secret_key), None, None, None).unwrap();
        let client = Client::new(endpoint, creds).unwrap();
        Self { client }
    }
}

#[async_trait]
impl BlobStorage for MinioStorage {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>, metadata: Metadata) -> Result<(), StorageError> {
        let bucket = BucketName::from(bucket);
        let object = ObjectName::from(key);
        self.client.put_object(&bucket, &object, data, metadata.map(|m| m.into())).await?;
        Ok(())
    }

    async fn get_object(&self, bucket: &str, key: &str) -> Result<(Vec<u8>, Metadata), StorageError> {
        let bucket = BucketName::from(bucket);
        let object = ObjectName::from(key);
        let (data, _) = self.client.get_object(&bucket, &object).await?;
        Ok((data, Metadata::default()))
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), StorageError> {
        let bucket = BucketName::from(bucket);
        let object = ObjectName::from(key);
        self.client.delete_object(&bucket, &object).await?;
        Ok(())
    }

    async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<ObjectInfo>, StorageError> {
        let bucket = BucketName::from(bucket);
        let objects = self.client.list_objects(&bucket, prefix).await?;
        let mut object_info = Vec::new();
        for obj in objects {
            let metadata = Metadata::default();
            object_info.push(ObjectInfo {
                key: obj.key,
                size: obj.size,
                last_modified: DateTime::from(obj.last_modified),
                metadata,
            });
        }
        Ok(object_info)
    }

    async fn update_metadata(&self, bucket: &str, key: &str, metadata: Metadata) -> Result<(), StorageError> {
        let bucket = BucketName::from(bucket);
        let object = ObjectName::from(key);
        self.client.put_object(&bucket, &object, Vec::new(), Some(metadata.into())).await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("MinIO error: {0}")]
    MinioError(#[from] minio::s3::error::Error),
}

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