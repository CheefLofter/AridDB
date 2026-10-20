


use database::{Database, SchemaType, DataValue};

#[macro_use]
mod database;

mod storage_engine;

fn main() {
    
    let db = Database::new(
        "example",
        vec![
            SchemaType::Integer,
            SchemaType::String, 
            SchemaType::Boolean],
    );
    println!("{}", db.push_one(row![1,"dsjal",true]).unwrap())

    
    
}
