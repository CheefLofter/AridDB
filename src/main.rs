

use crate::db::Database;
use std::fs;
use std::io;
use std::time::{Duration, Instant};

mod db;

fn add_rows_from_file(db: &Database, path: &str) -> io::Result<Duration> {
    let contents = fs::read_to_string(path)?;
    let rows: Vec<Vec<&str>> = contents.lines().map(|line| vec![line]).collect();

    let start = Instant::now();
    db.add_rows(&rows);
    Ok(start.elapsed())
}

fn main() -> io::Result<()> {
    let db = Database::new("example".into());

    let elapsed = add_rows_from_file(&db, "garbage_data.txt")?;
    println!("Added rows from file in {elapsed:?}");
    Ok(())
}
