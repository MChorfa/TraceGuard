use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::storage::blob_storage::{BlobStorage, Metadata, ObjectInfo, StorageError};

pub struct MockBlobStorage {
    objects: Mutex<HashMap<String, (Vec<u8>, Metadata)>>,
}

impl MockBlobStorage {
    pub fn new() -> Self {
        Self {
            objects: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl BlobStorage for MockBlobStorage {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>, metadata: Metadata) -> Result<(), StorageError> {
        let mut objects = self.objects.lock().unwrap();
        objects.insert(format!("{}/{}", bucket, key), (data, metadata));
        Ok(())
    }

    async fn get_object(&self, bucket: &str, key: &str) -> Result<(Vec<u8>, Metadata), StorageError> {
        let objects = self.objects.lock().unwrap();
        objects.get(&format!("{}/{}", bucket, key))
            .cloned()
            .ok_or(StorageError::NotFound)
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), StorageError> {
        let mut objects = self.objects.lock().unwrap();
        objects.remove(&format!("{}/{}", bucket, key));
        Ok(())
    }

    async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<ObjectInfo>, StorageError> {
        let objects = self.objects.lock().unwrap();
        let mut result = Vec::new();
        for ((b, k), (data, metadata)) in objects.iter() {
            if b == bucket && prefix.map_or(true, |p| k.starts_with(p)) {
                result.push(ObjectInfo {
                    key: k.to_string(),
                    size: data.len() as u64,
                    last_modified: metadata.created_at,
                    metadata: metadata.clone(),
                });
            }
        }
        Ok(result)
    }

    async fn update_metadata(&self, bucket: &str, key: &str, metadata: Metadata) -> Result<(), StorageError> {
        let mut objects = self.objects.lock().unwrap();
        if let Some((data, _)) = objects.get_mut(&format!("{}/{}", bucket, key)) {
            *_ = metadata;
            Ok(())
        } else {
            Err(StorageError::NotFound)
        }
    }
}