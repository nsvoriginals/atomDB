use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Eq)]
pub struct Row {
    pub data: HashMap<String, String>,
}

impl Row {
    pub fn new() -> Self {
        Row {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, column: String, value: String) {
        self.data.insert(column, value);
    }

    pub fn get(&self, column: &str) -> Option<&String> {
        self.data.get(column)
    }

    pub fn validate_columns(&self, required_columns: &[String]) -> bool {
        required_columns.iter().all(|col| self.data.contains_key(col))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Eq)]
pub struct Table {
    pub columns: Vec<String>,
    pub rows: HashMap<usize, Row>,
}

impl Table {
    pub fn new(columns: Vec<String>) -> Self {
        Table {
            columns,
            rows: HashMap::new(),
        }
    }

    pub fn insert_row(&mut self, row: Row) -> Result<usize, String> {
        if !row.validate_columns(&self.columns) {
            return Err("Row missing required columns".to_string());
        }

        let row_id = self.rows.len();
        self.rows.insert(row_id, row);
        Ok(row_id)
    }

    pub fn get_row(&self, id: usize) -> Option<&Row> {
        self.rows.get(&id)
    }

    pub fn get_all_rows(&self) -> Vec<(usize, &Row)> {
        self.rows.iter().map(|(id, row)| (*id, row)).collect()
    }

    pub fn select_where(&self, column: &str, value: &str) -> Vec<(usize, &Row)> {
        self.rows
            .iter()
            .filter(|(_, row)| {
                row.get(column).map_or(false, |v| v == value)
            })
            .map(|(id, row)| (*id, row))
            .collect()
    }
}
