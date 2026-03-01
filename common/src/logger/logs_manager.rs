use super::log_entry::LogEntry;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use chrono::{Datelike, Utc};



const MAGIC_NUMBER : &[u8] = b"Rust_log_V1";
const LOGS_SIZE : usize = 100;

pub struct LogsManager {
    path: String, // Path to save at.
    verbose : bool, // If the console receive or not the messages
    logs : Vec<LogEntry>, // Vector to save the data
}

impl LogsManager {
    pub fn new(name : &str, verbose : bool) -> LogsManager {
        
        // Take the time to create the file in the format "dd--m-yyyy"
        let now = Utc::now();
        let day : u8 = now.day().try_into().unwrap();
        let month : u8 = now.month().try_into().unwrap();
        let year : i32 = now.year().try_into().unwrap();
        
        let folder_path = format!("data/logs/{name}");
        let path : String = format!("{folder_path}/{day}-{month}-{year}");
        
        if let Err(e) = create_dir_all(&folder_path) {
            eprintln!("Unable to create the folder {} : {}", folder_path, e);
        }
        
        
        let l = LogsManager {
            path,
            verbose,
            logs :  Vec::with_capacity(LOGS_SIZE),
        };
        

        if !cfg!(debug_assertions) {
            match l.write_magic_number() {
                Ok(_) => return l,
                Err(_) => panic!("Unexcpeted error while creating the LogsManager")
            }
        }

        l
        
    }
    
    fn write_magic_number(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
        .create(true)
            .append(true)
            .open(format!("{}.bin", self.path))?;
        
        if file.metadata()?.len() == 0 {
            file.write_all(MAGIC_NUMBER)?;
        }
        
        Ok(())
    }
    
    fn save(&self) -> std::io::Result<()> {
        if cfg!(debug_assertions) {
            self.save_to_file()
        } else {
            self.save_to_file_binary()
        }
    }

    fn save_to_file_binary(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}.bin", self.path))?;

        for log in &self.logs {
            let encoded = bincode::serialize(log)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let len = encoded.len() as u64;

            file.write_all(&len.to_le_bytes())?;
            file.write_all(&encoded)?;
        }

        Ok(())
    }

    
    fn save_to_file(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        for log in &self.logs {
            let type_str : &'static str = (log.get_type()).into();
            writeln!(file, "[{}] {}", type_str, log.get_value())?;
        }
        Ok(())
    }

    fn append_log(&mut self, log : LogEntry) {
        self.logs.push(log);
    }

    fn get_logs_size(&self) -> usize {
        self.logs.len() // Return not instruction.
    }

    pub fn add_log(&mut self, log : LogEntry) {
        if self.verbose {log.print()};
        self.append_log(log);

        if self.get_logs_size() == LOGS_SIZE { // When the Vector is full -> flush it and save it into the log file.
            self.flush(); 
        }
    }

    
    fn flush(&mut self) {
        match &self.save() {
            Ok(()) => println!("Log saved successfully"),
            Err(_) => println!("{}", self.path),
        }
        self.logs.clear();
    }

    pub fn force_flush(&mut self) {
        self.flush();
    }
}

/* Prevent Log lost when compilator destruc the object */
impl Drop for LogsManager {
    fn drop(&mut self) {
        self.force_flush();

        if self.verbose {
            println!("LogsManager deleting : Flush automatique des dernières logs");
        }
    }
}