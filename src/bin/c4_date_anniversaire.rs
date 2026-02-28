use std::str::FromStr;

use chrono::{Datelike, Utc};
use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::console};


/// Represent a simplifided date
/// 
/// This struct use 4 bytes on the **stack** (u8 + u8 + i16)
struct Date {
    /// Range 1-31 (fits in u8)
    pub day : u8, 

    // Range 1-12 (fits in u8)
    pub month : u8, 

    /// Range -32.768 to 32.767 (sufficient for historical and future usage)
    pub year : i16,
}

impl FromStr for Date {
    type Err = ();

    /// Function to convert a str to a Date instance
    /// 
    /// # Parameters
    /// * `s` : the &str to convert into.
    /// 
    /// # Return
    /// * `Date` : the &str converted into a Date instance
    /// 
    /// # Author
    /// R3D
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // s.split() give back an iterator which is in on the **stack**
        let mut v = s.split('/');

        // v.next() consume the iterator
        // ok.or() Map a Some() | None into a Ok() | Err()
        // the first '?' operator is letting the code continue or get the error up.
        // .parse::<u8> convert the Ok() into a u8 a Result<> which is unwrapped with map_err() which define the err
        // the second '?' opearot is letting the coe continue or get the mapped error up.
        let day = v.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
        let month = v.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
        let year = v.next().ok_or(())?.parse::<i16>().map_err(|_| ())?;

        Ok(Date {day, month, year})
    }
}


///
/// Function to calculate the age of someone using Date struct
/// 
/// # Parameters
/// * `dob` : an instance of Date considirate like the Date Of Birth
///
/// # Return 
/// `i32` : the age from the instance date
/// 
/// # Author
/// R3D 
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
    l.add_log(log_entry::create_log(None, format!("Bienvenue {surname}")));

    let dob : Date = console::lire_type::<Date>(Some("Date de naissance ? "));

    l.add_log(log_entry::create_log(
        Some(LogType::Success), 
        format!(
            "Vous avez {} ans", 
            calculate_age(dob)
        )
    ));



}