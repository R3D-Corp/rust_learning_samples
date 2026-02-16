use crate::logger::log_type::LogType;

fn create_log(log_type : LogType, value : String) -> LogEntry {
    return LogEntry::new(log_type, value)
}

pub fn create_log_from_text(log_type : LogType, text : &str) -> LogEntry {
    return create_log(log_type, text.to_string());
}

pub fn create_log_string(log_type : LogType, text : String) -> LogEntry {
    return create_log(log_type, text)
}

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