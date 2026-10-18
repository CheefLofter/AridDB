

 
use std::fs::File;
use std::io::{self,BufRead, Write};
use std::path::Path;
use rev_buf_reader::RevBufReader;

pub struct Database {
    filename: String,
}

impl Database {
    pub fn new(filename: String) -> Self {
        let db = Self { filename: filename.clone() };

        if Path::new(&format!("{}.txt", filename)).exists() {
            return db;
        }

        File::create(&format!("{}.txt", filename)).unwrap();
        db
    }

   

    pub fn indexer(&self) -> io::Result<u32> {
        let file = match File::open(format!("{}.txt", self.filename)) {
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


    pub fn add_row(&self, row: Vec<&str>) {
        let mut file = File::options()
            .append(true)
            .open(&format!("{}.txt", &self.filename))
            .unwrap();

            let index = self.indexer().unwrap();
        writeln!(file, "{},{:?}",index, row).unwrap();
    }

    pub fn add_rows(&self, rows: &[Vec<&str>]) {
        if rows.is_empty() {
            return;
        }
       

        let mut file = File::options()
            .append(true)
            .open(&format!("{}.txt", &self.filename))
            .unwrap();

        let mut index = self.indexer().unwrap();

        let mut writer = io::BufWriter::new(&mut file);

        for row in rows {
            writeln!(writer, "{},{:?}", index, row).unwrap();
            index += 1;
        }

        writer.flush().unwrap();



    }
}