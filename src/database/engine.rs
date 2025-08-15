use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::schema::{Table, Row};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct StorageEngine {
    tables: HashMap<String, Table>,
}

impl StorageEngine {
    pub fn new() -> Self {
        StorageEngine {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &str, columns: Vec<String>) -> Result<(), String> {
        if self.tables.contains_key(name) {
            return Err(format!("Table '{}' already exists", name));
        }
        self.tables.insert(
            name.to_string(),
            Table::new(columns)
        );
        Ok(())
    }

    pub fn insert_row(&mut self, table_name: &str, row: Row) -> Result<usize, String> {
        match self.tables.get_mut(table_name) {
            Some(table) => table.insert_row(row),
            None => Err(format!("Table '{}' not found", table_name))
        }
    }

    pub fn select_all(&self, table_name: &str) -> Result<Vec<(usize, &Row)>, String> {
        match self.tables.get(table_name) {
            Some(table) => Ok(table.get_all_rows()),
            None => Err(format!("Table '{}' not found", table_name))
        }
    }

    pub fn select_where(&self, table_name: &str, column: &str, value: &str) -> Result<Vec<(usize, &Row)>, String> {
        match self.tables.get(table_name) {
            Some(table) => Ok(table.select_where(column, value)),
            None => Err(format!("Table '{}' not found", table_name))
        }
    }

    pub fn describe_table(&self, name: &str) -> Result<&Vec<String>, String> {
        match self.tables.get(name) {
            Some(table) => Ok(&table.columns),
            None => Err(format!("Table '{}' not found", name))
        }
    }

    pub fn list_tables(&self) -> Vec<&String> {
        self.tables.keys().collect()
    }

    pub fn drop_table(&mut self, name: &str) -> Result<(), String> {
        match self.tables.remove(name) {
            Some(_) => Ok(()),
            None => Err(format!("Table '{}' not found", name))
        }
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn deserialize(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn serialize_binary(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn deserialize_binary(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.serialize()?;
        std::fs::write(filename, json)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filename)?;
        let db = Self::deserialize(&json)?;
        Ok(db)
    }

    pub fn save_to_binary_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let binary_data = self.serialize_binary()?;
        std::fs::write(filename, binary_data)?;
        Ok(())
    }

    pub fn load_from_binary_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let binary_data = std::fs::read(filename)?;
        let db = Self::deserialize_binary(&binary_data)?;
        Ok(db)
    }
}

impl Default for StorageEngine {
    fn default() -> Self {
        Self::new()
    }
}
