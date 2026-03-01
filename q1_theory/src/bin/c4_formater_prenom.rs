use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::console};

/// Function that receive a string slice and format it with the first letter in uppercase & other one in lowercase.
/// 
/// # Parameters
/// * `s` : A string slice (`&str`) to be formatted. This is a borrowed value, 
///         so the original string is not consumed.
/// 
/// # Return
/// * `String` : A new, allocated String containing the formatted result.
///
/// # Author
/// R3D
fn first_letter_uppercase(s : &str) -> String{
    let mut result = String::with_capacity(s.len());
    
    if s.is_empty() { return result}
    
    let mut chars = s.chars();
    let first = match chars.next()  {
        Some(c) => c,
        None => return result,
    };

    for upper in first.to_uppercase() {
        result.push(upper);
    }

    for c in chars {
        for lower in c.to_lowercase() {
            result.push(lower);
        }
    }
    result
}


fn main() {
    let mut l : LogsManager = LogsManager::new("c4_formater_prenom", true);

    let name = first_letter_uppercase(&console::lire_string_msg("Nom ? "));
    let firstname = first_letter_uppercase(&console::lire_string_msg("Prénom ? ")); 
    let city = first_letter_uppercase(&console::lire_string_msg("Ville ? "));
    l.add_log(log_entry::create_log(Some(LogType::Success), format!("{} {} {}", name, firstname, city)));
}