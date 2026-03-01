use std::{fmt::Display, str::FromStr};

use helmo_b1_rust::tools::console;
use regex::Regex;


/// 
/// Function to calculate the verfication numbers used to validate a National Register number
/// 
/// # Parameters 
/// `register_value` : an National register value.
/// 
/// # Return
/// `(u8, u8)` : both verfication number it represent two old number and the new number(born before 2000, born after 2000)
/// 
fn calculate_correct_numbers(register_value : u64) -> (u8, u8) {
    let first = (97 - register_value % 97) as u8;
    let second = (97 - (register_value as u64 + 2e9 as u64) % 97) as u8;

    (first, second)
}


// A basic Register representation
struct Register {
    fields : (u8, u8, u8, u16, u8), // Before I used 5 fields instead I grouped them in a tuple.
}

impl Register {

    ///
    /// Function to determine if the Register instance is valid.
    /// 
    /// # Return
    /// `bool` : the Register validity.
    /// 
    fn is_valid(&self) -> bool {
        let n_registre: u64 = (self.fields.0 as u64 * 10_000_000) // AA
                            + (self.fields.1 as u64 * 100_000)                     // MM
                            + (self.fields.2 as u64 * 1_000)                        // JJ
                            + (self.fields.3 as u64);    

        let validity_number = self.fields.4;

        if validity_number < 1 || validity_number > 97 {
            return false;
        }

        let (first, second) : (u8, u8) = calculate_correct_numbers(n_registre);
        self.fields.4 == first || self.fields.4 == second
    }
}

impl FromStr for Register {
    type Err = ();

    ///
    /// Function to convert from &str an Register
    /// It's used in the `console::lire_type<T>` below
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d{2})\.(\d{2})\.(\d{2})-(\d{3})\.(\d{2})$").unwrap();

        if !re.is_match(s) {
            return Err(());
        }
        
        let caps = re.captures(s).ok_or(())?;
        Ok(Register {
            fields : (
                caps[1].parse().map_err(|_| ())?, 
                caps[2].parse().map_err(|_| ())?, 
                caps[3].parse().map_err(|_| ())?, 
                caps[4].parse().map_err(|_| ())?, 
                caps[5].parse().map_err(|_| ())?
            )
        })
    }
}

impl Display for Register {
    /// 
    /// Function to display a Register. it is used to not stock an heavy String
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:02}.{:02}.{:02}-{:03}.{:02}", self.fields.0, self.fields.1, self.fields.2, self.fields.3, self.fields.4) }
}

fn main() {
    let registre = console::lire_type::<Register>(Some("Votre registre"));
    let response = match registre.is_valid() {
        true => "est valide",
        false => "n'est pas valide"
    };

    println!("le registre {} : {}", registre, response)
}