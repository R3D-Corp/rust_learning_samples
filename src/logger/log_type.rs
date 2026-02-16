use super::ansi_colors::AnsiColors;

#[allow(dead_code)]
pub enum LogType {
    Info,
    Warning,
    Error,
    Success
}

impl LogType {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
            Self::Success => "SUCCESS"
        }
    }

    fn get_color(&self) -> &str {
        match self {
            Self::Info => AnsiColors::Blue.to_str(),
            Self::Warning => AnsiColors::Yellow.to_str(),
            Self::Error => AnsiColors::Red.to_str(),
            Self::Success => AnsiColors::Green.to_str()
        }
    }
}

impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = self.get_color();
        write!(f, "{}{}{}", color, self.to_str(), AnsiColors::Reset.to_str())
    }
}