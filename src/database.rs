

#![allow(dead_code)] //for anoying compiler warnings


use std::path::Path;
use std::fs::File;
use std::io;
use crate::storage_engine::StorageEngine;

use serde_json::Value;

#[derive(Debug)]
pub enum SchemaType {
    Integer,
   
    String,
    Boolean,
}

pub struct Database {
    schema: Vec<SchemaType>,
    database: String,
}

impl Database {

   

    pub fn new(db_name: &str, schema: Vec<SchemaType>) -> Self {

        let db = Self {
            schema: schema,
            database: db_name.to_string(),
        };

         if Path::new(&format!("{}.dbstf", db_name)).exists() {
            return db;
        }

        File::create(&format!("{}.dbstf", db_name)).unwrap();
        db

   }

    fn schema_validator(&self, row: &Value) -> bool {
        let Some(values) = row.as_array() else {
            return false;
        };

        if values.len() != self.schema.len() {
            return false;
        }

        for (schema_type, value) in self.schema.iter().zip(values) {
            match (schema_type, value) {
                (SchemaType::Integer, Value::Number(number))
                    if number.is_i64() || number.is_u64() => continue,
                
                (SchemaType::String, Value::String(_)) => continue,
                (SchemaType::Boolean, Value::Bool(_)) => continue,
                _ => return false,
            }
        }

        true
    }

    pub fn push_one(&self, row: Value) -> io::Result<String> {
        if self.schema_validator(&row) {
            let json_str = serde_json::to_string(&row)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;


            StorageEngine::add_row(&self.database, json_str);
            Ok("success".to_string())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid schema"))
        }
    }

    pub fn get_one(&self, index: u32) -> io::Result<Value> {
        let row = StorageEngine::read_row(&self.database, index)?;
        serde_json::from_str(&row)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn push_many(&self, rows: &[Value]) -> io::Result<String> {


        if rows.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No rows provided"));
        }
        let mut data_to_str_vec = Vec::new();

        for row in rows {
            if !self.schema_validator(row) {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid schema"))
            }
            
            for row in rows {
                let json_str = serde_json::to_string(row)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                data_to_str_vec.push(json_str);
            }

            
        }
        StorageEngine::add_rows(&self.database, data_to_str_vec);
        return Ok("success".to_string())
    }

    pub fn get_many(&self, start_index: u32, end_index: u32) -> io::Result<Vec<Value>> {
        let rows = StorageEngine::read_rows(&self.database, start_index, end_index)?;
        let mut result = Vec::new();

        for row in rows {
            let value: Value = serde_json::from_str(&row)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            result.push(value);
        }

        Ok(result)
    }


    pub fn dump(&self) -> io::Result<Vec<Value>> {
        let rows = StorageEngine::dump_db(&self.database)?;
        let mut result = Vec::new();

        for row in rows {
            let value: Value = serde_json::from_str(&row)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            result.push(value);
        }

        Ok(result)
    }
}