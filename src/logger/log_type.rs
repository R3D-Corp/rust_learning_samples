use serde::{Deserialize, Serialize};

use super::ansi_colors::AnsiColors;


pub const DEFAULT_LOG_TYPE : LogType = LogType::Info;

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum LogType {
    Info,
    Warning,
    Error,
    Success
}

impl LogType {
    fn get_color(&self) -> &str {
        match self {
            Self::Info => AnsiColors::Blue.to_str(),
            Self::Warning => AnsiColors::Yellow.to_str(),
            Self::Error => AnsiColors::Red.to_str(),
            Self::Success => AnsiColors::Green.to_str()
        }
    }
}

impl Default for LogType {
    fn default() -> Self {
        DEFAULT_LOG_TYPE
    }
}

impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = self.get_color();
        let log : &'static str = self.into();

        write!(f, "{}{}{}", color, log, AnsiColors::Reset.to_str())
    }
}

/* Convert Traits */
impl From<&LogType> for &'static str {
    fn from(log : &LogType) -> Self {
        match log {
            LogType::Info => "INFO",
            LogType::Warning => "WARNING",
            LogType::Error => "ERROR",
            LogType::Success => "SUCCESS"
        }
    }
}

impl From<LogType> for &'static str {
    fn from(log : LogType) -> Self {
        match log {
            LogType::Info => "INFO",
            LogType::Warning => "WARNING",
            LogType::Error => "ERROR",
            LogType::Success => "SUCCESS"
        }
    }
}