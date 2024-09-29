use apache_iceberg::catalog::Catalog;
use apache_iceberg::table::Table;

pub struct IcebergMetadata {
    catalog: Box<dyn Catalog>,
}

impl IcebergMetadata {
    pub fn new(catalog: Box<dyn Catalog>) -> Self {
        Self { catalog }
    }

    pub fn create_table(&self, table_name: &str, schema: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.catalog.create_table(table_name, schema)?;
        Ok(())
    }

    pub fn get_table(&self, table_name: &str) -> Result<Table, Box<dyn std::error::Error>> {
        let table = self.catalog.load_table(table_name)?;
        Ok(table)
    }

    pub fn update_metadata(&self, table: &Table, metadata: &str) -> Result<(), Box<dyn std::error::Error>> {
        table.update_properties(metadata)?;
        Ok(())
    }
}