use std::{fmt::Display, str::FromStr};
use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}};

struct Ip {
    first : u8,
    second : u8,
    third : u8,
    fourth : u8
}


impl Ip {
    fn new(first : &str, second : &str, third : &str, fourth : &str) -> Ip {
        Ip {
            first: convert_byte(first),
            second: convert_byte(second),
            third: convert_byte(third),
            fourth: convert_byte(fourth)
        }
    }
}

impl FromStr for Ip {
    type Err = ();

    fn from_str(s : &str) -> Result<Ip, ()> {
        let v : Vec<&str> = s.split('.').collect();

        if v.len() != 4 {
            return Err(());
        }

        Ok(Ip::new(v[0], v[1], v[2], v[3]))
    }
}

impl Display for Ip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.first, self.second, self.third, self.fourth)
    }
}

fn convert_byte(byte : &str) -> u8 {
    byte.as_bytes().iter().fold(0, |acc, &b| {
        match b {
            b'0' => acc << 1,
            b'1' => (acc << 1) | 1,
            _ => acc,
        }
    })
}


fn main() {

    let mut l : LogsManager = LogsManager::new("c5_adresse_ip_en_base_10", true);

    let ip : &str = "11000000.10101000.00000001.00000101";
    let ip = Ip::from_str(ip).unwrap_or(Ip::new("0", "0", "0", "0"));
    let message = format!("{}", ip);

    l.add_log(log_entry::create_log(Some(LogType::Success), message));

}