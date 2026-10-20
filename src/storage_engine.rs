
// #![allow(dead_code)] //for anoying compiler warnings

/*
 indexing is based on u32 which means the max number of rows is 4,294,967,295. 
 this more than enought for this db.

 thi db doesnot implement a b-tree just beacuse it is meant to be a simple to use and niche db-
-meant where the setup is small and just need to save arrays of data.


*/
 
use std::fs::File;
use std::io::{self,BufRead, Write};
use std::path::Path;
use rev_buf_reader::RevBufReader;



pub struct StorageEngine {
    filename: String,
}

impl StorageEngine {
    

   

    pub fn indexer(filename: &str) -> io::Result<u32> {
        let file = match File::open(format!("{}.txt", filename)) {
            Ok(f) => f,
            // Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(0), // no file yet
            Err(e) => return Err(e),
        };
        let reader = RevBufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue; // blank line, look further up
            }

            // "12,[a,b,c]" -> "12"
            if let Some((idx, _rest)) = line.split_once(',') {
                if let Ok(n) = idx.trim().parse::<u32>() {
                    return Ok(n + 1);
                }
            }
            // no comma or non-numeric index: treat as a bad line and keep going up
        }
        println!("Indexer called, returning 0");
        Ok(0) // empty file or no valid row found
        
    }


    pub fn add_row(filename: &str, row: String) {
        let mut file = File::options()
            .append(true)
            .open(&format!("{}.txt", filename))
            .unwrap();

            let index = StorageEngine::indexer(filename).unwrap();
        writeln!(file, "{},{}",index, row).unwrap();
    }

    pub fn add_rows(filename: &str, rows: &[Vec<&str>]) {
        if rows.is_empty() {
            return;
        }
       

        let mut file = File::options()
            .append(true)
            .open(&format!("{}.txt", filename))
            .unwrap();

        let mut index = StorageEngine::indexer(filename).unwrap();

        let mut writer = io::BufWriter::new(&mut file);

        for row in rows {
            writeln!(writer, "{},{:?}", index, row).unwrap();
            index += 1;
        }

        writer.flush().unwrap();



    }

    pub fn read_row(filename: &str, index: u32) -> io::Result<Vec<String>> {
        let file = File::open(format!("{}.txt", filename))?;
        let reader = io::BufReader::new(file);


        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue; // skip blank lines
            }

            if let Some((idx, rest)) = line.split_once(',') {
                if let Ok(n) = idx.trim().parse::<u32>() {
                    // Remove the brackets and split by comma
                    if n == index {
                        let row_str = rest.trim();
                        let row_str = row_str.trim_start_matches('[').trim_end_matches(']');
                        let row: Vec<String> = row_str.split(',').map(|s| s.trim().to_string()).collect();
                        return Ok(row);
                    }
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Row not found"))
    }

    pub fn read_rows(filename: &str, start_index: u32, end_index: u32) -> io::Result<Vec<Vec<String>>> {
        let file = File::open(format!("{}.txt", filename))?;
        let reader = io::BufReader::new(file);
        let mut rows = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue; // skip blank l
            }

            if let Some((idx, rest)) = line.split_once(',') {
                if let Ok(n) = idx.trim().parse::<u32>() {
                    if n >= start_index && n <= end_index {
                        // Remove the brackets and split by comma
                        let row_str = rest.trim();
                        let row_str = row_str.trim_start_matches('[').trim_end_matches(']');
                        let row: Vec<String> = row_str.split(',').map(|s| s.trim().to_string()).collect();
                        rows.push(row);
                    }
                }
            }
        }

        Ok(rows)
    }



}