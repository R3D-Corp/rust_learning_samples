use serde::{Deserialize, Serialize};

use crate::logger::log_type::LogType;

fn generate_log(log_type : LogType, value : String) -> LogEntry {
    return LogEntry::new(log_type, value)
}

#[deprecated(note = "Use `create_log` instead.")]
pub fn create_log_from_text(log_type : LogType, text : &str) -> LogEntry {
    return generate_log(log_type, text.to_string());
}

#[deprecated(note = "Use `create_log` instead.")]
pub fn create_log_string(log_type : LogType, text : String) -> LogEntry {
    return generate_log(log_type, text)
}

pub fn create_log<S: Into<String>>(log_type : Option<LogType>, text : S) -> LogEntry {
    let t = log_type.unwrap_or_default();
    generate_log(t, text.into())
}   

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    log_type : LogType,
    value : String,
}

impl LogEntry {
    pub fn print(&self) {
        let message = format!("[{}] : {}", self.log_type, self.value);
        println!("{}", message);
    }

    pub fn get_type(&self) -> &LogType {
        &self.log_type
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn new(log_type : LogType, value : String) -> LogEntry {
        LogEntry {
            log_type : log_type,
            value : value,
        }
    }
}
