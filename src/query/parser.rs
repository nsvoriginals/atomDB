use crate::database::{StorageEngine, Row};

pub struct QueryEngine<'a> {
    storage: &'a mut StorageEngine,
}

impl<'a> QueryEngine<'a> {
    pub fn new(storage: &'a mut StorageEngine) -> Self {
        QueryEngine { storage }
    }

    pub fn execute(&mut self, query: &str) -> Result<String, String> {
        let query = query.trim().to_lowercase();
        let parts: Vec<&str> = query.split_whitespace().collect();
        
        match parts.get(0) {
            Some(&"create") => self.handle_create(&parts[1..]),
            Some(&"insert") => self.handle_insert(&parts[1..]),
            Some(&"select") => self.handle_select(&parts[1..]),
            Some(&"describe") => self.handle_describe(&parts[1..]),
            Some(&"show") => self.handle_show(&parts[1..]),
            Some(&"drop") => self.handle_drop(&parts[1..]),
            _ => Err("Unknown command".to_string()),
        }
    }

    fn handle_create(&mut self, parts: &[&str]) -> Result<String, String> {
        if parts.len() < 4 || parts[0] != "table" {
            return Err("Invalid CREATE TABLE syntax".to_string());
        }

        let table_name = parts[1];
        let columns_str = parts[2..].join(" ");

        if !columns_str.starts_with('(') || !columns_str.ends_with(')') {
            return Err("Columns must be in parentheses".to_string());
        }

        let columns_part = &columns_str[1..columns_str.len() - 1];
        let columns: Vec<String> = columns_part
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        self.storage.create_table(table_name, columns)?;
        Ok(format!("Table '{}' created successfully", table_name))
    }

    fn handle_insert(&mut self, parts: &[&str]) -> Result<String, String> {
        if parts.len() < 3 || parts[0] != "into" {
            return Err("Invalid INSERT syntax".to_string());
        }

        let table_name = parts[1];
        let values_str = parts[2..].join(" ");

        if !values_str.starts_with('(') || !values_str.ends_with(')') {
            return Err("Values must be in parentheses".to_string());
        }

        let values_part = &values_str[1..values_str.len() - 1];
        let mut row = Row::new();

        for pair in values_part.split(',') {
            let kv: Vec<&str> = pair.trim().split('=').collect();
            if kv.len() != 2 {
                return Err("Invalid key=value format".to_string());
            }
            row.insert(kv[0].trim().to_string(), kv[1].trim().to_string());
        }

        let row_id = self.storage.insert_row(table_name, row)?;
        Ok(format!("Row inserted with ID: {}", row_id))
    }

    fn handle_select(&mut self, parts: &[&str]) -> Result<String, String> {
        if parts.len() < 3 || parts[0] != "*" || parts[1] != "from" {
            return Err("Invalid SELECT syntax".to_string());
        }

        let table_name = parts[2];

        let rows = if parts.len() > 3 && parts[3] == "where" {
            if parts.len() < 5 {
                return Err("Invalid WHERE clause".to_string());
            }
            let condition = parts[4];
            let condition_parts: Vec<&str> = condition.split('=').collect();
            if condition_parts.len() != 2 {
                return Err("Invalid WHERE condition".to_string());
            }
            self.storage.select_where(table_name, condition_parts[0], condition_parts[1])?
        } else {
            self.storage.select_all(table_name)?
        };

        let mut result = format!("Results from table '{}':\n", table_name);
        for (id, row) in rows {
            result.push_str(&format!("ID: {} | {:?}\n", id, row.data));
        }
        Ok(result)
    }

    fn handle_describe(&mut self, parts: &[&str]) -> Result<String, String> {
        if parts.is_empty() {
            return Err("Table name required".to_string());
        }
        let table_name = parts[0];
        let columns = self.storage.describe_table(table_name)?;
        Ok(format!("Table '{}' columns: {:?}", table_name, columns))
    }

    fn handle_show(&mut self, parts: &[&str]) -> Result<String, String> {
        if parts.get(0) == Some(&"tables") {
            let tables = self.storage.list_tables();
            Ok(format!("Tables: {:?}", tables))
        } else {
            Err("Only 'SHOW TABLES' is supported".to_string())
        }
    }

    fn handle_drop(&mut self, parts: &[&str]) -> Result<String, String> {
        if parts.len() < 2 || parts[0] != "table" {
            return Err("Invalid DROP TABLE syntax".to_string());
        }
        let table_name = parts[1];
        self.storage.drop_table(table_name)?;
        Ok(format!("Table '{}' dropped successfully", table_name))
    }
}
