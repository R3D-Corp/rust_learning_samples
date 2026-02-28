use std::{fmt::Display, str::FromStr};

use helmo_b1_rust::tools::console;
use regex::Regex;

fn calculate_correct_numbers(register_value : u64) -> (u8, u8) {
    let first = (97 - register_value % 97) as u8;
    let second = (97 - (register_value as u64 + 2e9 as u64) % 97) as u8;

    (first, second)
}

struct Register {
    first_section: u8,
    second_section : u8,
    third_section : u8,
    fourth_section : u16,
    fifth_section : u8
}

impl Register {
    fn is_valid(&self) -> bool {
        let n_registre: u64 = (self.first_section as u64 * 10_000_000) // AA
                            + (self.second_section as u64 * 100_000)                     // MM
                            + (self.third_section as u64 * 1_000)                        // JJ
                            + (self.fourth_section as u64);    

        let validity_number = self.fifth_section;

        if validity_number < 1 || validity_number > 97 {
            return false;
        }

        let (first, second) : (u8, u8) = calculate_correct_numbers(n_registre);
        self.fifth_section == first || self.fifth_section == second
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d{2})\.(\d{2})\.(\d{2})-(\d{3})\.(\d{2})$").unwrap();

        if !re.is_match(s) {
            return Err(());
        }
        
        let caps = re.captures(s).ok_or(())?;
        Ok(Register {
            first_section: caps[1].parse().map_err(|_| ())?,
            second_section: caps[2].parse().map_err(|_| ())?,
            third_section: caps[3].parse().map_err(|_| ())?,
            fourth_section: caps[4].parse().map_err(|_| ())?,
            fifth_section: caps[5].parse().map_err(|_| ())?,
        })
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:02}.{:02}.{:02}-{:03}.{:02}", self.first_section, self.second_section, self.third_section, self.fourth_section, self.fifth_section) }
}

fn main() {
    let registre = console::lire_type::<Register>(Some("Votre registre"));
    let response = match registre.is_valid() {
        true => "est valide",
        false => "n'est pas valide"
    };

    println!("le registre {} : {}", registre, response)
}