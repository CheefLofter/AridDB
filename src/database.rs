use std::path::Path;
use std::fs::File;
use std::io;
use crate::storage_engine::StorageEngine;

use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Serialize, Deserialize, Debug)]
struct Container {
    items: Vec<Value>, // Holds any JSON type
}





#[derive(Debug)]
pub enum SchemaType {
    Integer,
    Float,
    String,
    Boolean,
}


#[derive(Debug, Serialize)]
pub enum DataValue {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
   
}


impl From<i32> for DataValue {
    fn from(value: i32) -> Self {
        DataValue::Int(value)
    }
}

impl From<&str> for DataValue {
    fn from(value: &str) -> Self {
        DataValue::String(value.to_string())
    }
}

impl From<bool> for DataValue {
    fn from(value: bool) -> Self {
        DataValue::Bool(value)
    }
}

macro_rules! row {
    ($($value:expr),* $(,)?) => {
        vec![$($crate::database::DataValue::from($value)),*]
    };
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

         if Path::new(&format!("{}.txt", db_name)).exists() {
            return db;
        }

        File::create(&format!("{}.txt", db_name)).unwrap();
        db

   }


    fn make_json_str(self, row: Vec<DataValue>) -> String{

        
        let json_string = serde_json::to_string(&row).unwrap();
        println!("String: {}", json_string);
        json_string

    }
    


    fn schema_validater(&self, row: &Vec<DataValue>) -> bool {
        if row.len() != self.schema.len() {
            return false;
        }

        for (i, value) in row.iter().enumerate() {
            match (&self.schema[i], value) {
                (SchemaType::Integer, DataValue::Int(_)) => continue,
                (SchemaType::Float, DataValue::Float(_)) => continue,
                (SchemaType::String, DataValue::String(_)) => continue,
                (SchemaType::Boolean, DataValue::Bool(_)) => continue,
                _ => return false,
            }
        }

        true
    }


    pub fn push_one(&self, row: Vec<DataValue>) -> io::Result<String> {

        if self.schema_validater(&row){

            let json_str = serde_json::to_string(&row).unwrap();
            StorageEngine::add_row(&self.database, json_str);
            Ok("success".to_string())


        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid schema"))
        }

    }

}