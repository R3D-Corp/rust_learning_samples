#[allow(dead_code)]
pub enum AnsiColors {
    Reset,
    Bold,
    Red,
    Green,
    Yellow,
    Blue,
    Purple
}

impl AnsiColors {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Reset =>  "\u{001B}[0m",
            Self::Bold => "\u{001B}[1m",
            Self::Red => "\u{001B}[31m",
            Self::Green => "\u{001B}[32m",
            Self::Yellow => "\u{001B}[33m",
            Self::Blue => "\u{001B}[34m",
            Self::Purple => "\u{001B}[35m",
        }
    }
}