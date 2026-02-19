use std::str::FromStr;

use chrono::{Datelike, Utc};
use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::console};


struct Date {
    pub day : u8,
    pub month : u8, 
    pub year : i16
}

impl FromStr for Date {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('/').collect();

        if v.len() != 3 {
            return Err(());
        }


        let day : u8 = v[0].parse().map_err(|_| ())?;
        let month : u8 = v[1].parse().map_err(|_| ())?;
        let year : i16 = v[2].parse().map_err(|_| ())?;

        Ok(Date {day, month, year})
    }
}

fn calculate_age(dob : Date) -> i32 {
    let now = Utc::now();

    let day : u8 = now.day().try_into().unwrap();
    let month : u8 = now.month().try_into().unwrap();
    let year = now.year();
    
    let mut age= year - (dob.year) as i32;

    if month < dob.month || (month == dob.month && day < dob.day) {
        age -= 1;
    }
    age
}

fn main() {
    let mut l = LogsManager::new("date_anniversaire", true);
    
    let surname : String = console::lire_string_msg("Prénom ? ");
    l.add_log(log_entry::create_log_string(LogType::Info, format!("Bienvenue {surname}")));

    let dob : Date = console::lire_type::<Date>(Some("Date de naissance ? "));

    l.add_log(log_entry::create_log_string(
        LogType::Success, 
        format!(
            "Vous avez {} ans", 
            calculate_age(dob)
        )
    ));



}