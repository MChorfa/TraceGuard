use aws_sdk_s3 as s3;
use apache_iceberg::catalog::Catalog;
use apache_iceberg::table::Table;
use crate::error::AppError;

pub struct Storage {
    s3_client: s3::Client,
    iceberg_catalog: Catalog,
}

impl Storage {
    pub async fn new() -> Result<Self, AppError> {
        let s3_client = s3::Client::new(&s3::Config::default());
        let iceberg_catalog = Catalog::new("your_catalog_name").await?;

        Ok(Self {
            s3_client,
            iceberg_catalog,
        })
    }

    pub async fn store_sbom(&self, sbom: &[u8], key: &str) -> Result<(), AppError> {
        self.s3_client
            .put_object()
            .bucket("your-sbom-bucket")
            .key(key)
            .body(sbom.to_vec().into())
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_sbom(&self, key: &str) -> Result<Vec<u8>, AppError> {
        let response = self.s3_client
            .get_object()
            .bucket("your-sbom-bucket")
            .key(key)
            .send()
            .await?;

        Ok(response.body.collect().await?.into_bytes().to_vec())
    }

    pub async fn update_metadata(&self, table_name: &str, metadata: &serde_json::Value) -> Result<(), AppError> {
        let table = self.iceberg_catalog.load_table(table_name).await?;
        table.update_properties(metadata).await?;
        Ok(())
    }

    pub async fn query_metadata(&self, table_name: &str, query: &str) -> Result<Vec<serde_json::Value>, AppError> {
        let table = self.iceberg_catalog.load_table(table_name).await?;
        let result = table.scan().filter(query).collect().await?;
        Ok(result)
    }
}