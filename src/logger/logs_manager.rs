use super::log_entry::LogEntry;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use chrono::{Datelike, Utc};

pub struct LogsManager {
    path: String,
    verbose : bool,
    logs : Vec<LogEntry>

}

const LOGS_SIZE : usize = 100;
impl LogsManager {
    pub fn new(name : &str, verbose : bool) -> LogsManager {
        let now = Utc::now();
        let day : u8 = now.day().try_into().unwrap();
        let month : u8 = now.month().try_into().unwrap();
        let year : i32 = now.year().try_into().unwrap();

        let folder_path = format!("data/logs/{name}");
        let path : String = format!("{folder_path}/{day}-{month}-{year}");

        if let Err(e) = create_dir_all(&folder_path) {
            eprintln!("Unable to create the folder {} : {}", folder_path, e);
        }

        LogsManager {
            path,
            verbose,
            logs :  Vec::with_capacity(LOGS_SIZE)
        }
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        for log in &self.logs {
            writeln!(file, "[{}] {}", log.get_type().to_str(), log.get_value())?;
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
        match &self.save_to_file() {
            Ok(()) => println!("Log saved successfully"),
            Err(_) => println!("{}", self.path),
        }
        self.logs.clear();
    }

    pub fn force_flush(&mut self) {
        self.flush();
    }
}

impl Drop for LogsManager {
    fn drop(&mut self) {
        self.force_flush();

        if self.verbose {
            println!("LogsManager deleting : Flush automatique des dernières logs");
        }
    }
}