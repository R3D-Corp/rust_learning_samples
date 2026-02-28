use std::fmt::Display;

use helmo_b1_rust::tools::console;


///
/// Enumerator that give every supported country.
pub enum Countries {
    Netherlands,
    Belgium,
    France,
    Spain,
    Portugal,
    Ivory,
    Unset
}

impl Countries {
    // Method to convert a &str to a Countries::
    fn from_number(c : &str) -> Countries {
        if c.starts_with("+31") || c.starts_with("0031") { return Countries::Netherlands }
        if c.starts_with("+32") || c.starts_with("0032") { return Countries::Belgium }
        if c.starts_with("+33") || c.starts_with("0033") { return Countries::France }
        if c.starts_with("+34") || c.starts_with("0034") { return Countries::Spain }
        if c.starts_with("+35") || c.starts_with("0035") { return Countries::Portugal }
        if c.starts_with("+225") || c.starts_with("0225") { return Countries::Ivory }
        Countries::Unset
    }
}

impl Display for Countries {
    // Method to display a country name
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}

impl From<&Countries> for &str {
    // Method to convert  a borrowed Countries:: and get the country name in &str
    fn from(c: &Countries) -> Self {
            match c {
                Countries::Netherlands => "Pays-Bas",
                Countries::Belgium => "Belgique",
                Countries::France => "France",
                Countries::Spain => "Espagne",
                Countries::Portugal => "Portugal",
                Countries::Ivory => "Côte d'ivoire",
                _ => "Inconnu"
            }
    }
}

impl From<Countries> for &str {
    // Method to convert Countries:: and get the country name in &str
    fn from(c: Countries) -> Self {
        Self::from(&c)
    }
}


fn main() {
    let phone_number = console::lire_string_msg("Numéro international ? ");

    let country = Countries::from_number(&phone_number);
    println!("{}", country);
}