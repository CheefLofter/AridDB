

/*
    The main file is here for an example porpose and is not need for the project. 
    - you can add database.rs and storage_engine.rs to your project and use them.
    schema validation in done in db.rs and sent for storage
    - everything stored in a .dbstf file and is readable txt not stored as binary
*/

use database::{Database, SchemaType};

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
   
    // push single row
     match db.push_one(       
        serde_json::json!([1, "Hello", true])
    ){
        Ok(msg) => println!("Push one result: {}", msg),
        Err(e) => println!("Error: {}", e),
     }
     // pushing mmultiple rows to the db
     match db.push_many(&[
        serde_json::json!([2, "World", false]),
        serde_json::json!([3, "Rust", true])
    ]){
        Ok(msg) => println!("Push many result: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
    // geting a single row
    match db.get_one(1) {
        Ok(row) => println!("Get one result: {}", row),
        Err(e) => println!("Error: {}", e),
    }
    // geting a range of rows
    match db.get_many(1, 3) {
        Ok(rows) => {
            println!("Get many result:");
            for row in rows {
                println!("{}", row);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
    // dumping entire db
    match db.dump() {
        Ok(rows) => {
            println!("Dump result:");
            for row in rows {
                println!("{}", row);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}




    

    
