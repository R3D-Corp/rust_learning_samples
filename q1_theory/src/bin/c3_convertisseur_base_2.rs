use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::console};



/// Convert a natural integer ('usize') in its binary representation
/// 
/// This function uses bitwise operations to extract every bit
/// in an optimal way without doing divisions
/// 
/// # Arguments
/// * `n` The integer of type 'uszize' to convert
/// 
/// # Return
/// * `Vec<u8>`:  the bits vector.
/// 
/// # Author
/// * R3D
fn convertisseur_base_2(mut n : usize) -> Vec<u8> {
    
    let mut binary : Vec<u8> = Vec::with_capacity(usize::BITS as usize); // Pre-allocate memory to avoid multiple reallocations.

    if n == 0 { // If the user input is 0 we don't want the code to go in the while.
        return vec![0]; 
    }
    
    while n > 0 {
        binary.push((n & 1) as u8); // Check if last bit n equal 1.
        n >>= 1; // Move alls bits to right = divide by two.
    }

    binary.reverse();
    binary
}

/// Convert a natural integer ('usize') in its hexadecimal representation
/// 
/// This function uses bitwise operations to extract every bit
/// in an optimal way without doing divisions
/// 
/// # Arguments
/// * `n` The integer of type 'uszize' to convert
/// 
/// # Return
/// * `String`:  the hexadecimal representation.
/// 
/// # Author
/// * R3D
fn convertisseur_base_16(mut n : usize) -> String {
    if n == 0 { return "0".to_string() };

    let mut hex = String::with_capacity(16);

    
    while n > 0 {
        let rem = (n & 15) as u8;
        
        let ch = match rem {
            0..=9 => (rem + b'0') as char,
            10..=15 => (rem - 10 + b'A') as char,
            _ => unreachable!(),
        };
        hex.push(ch);
        n >>= 4; // Move 4bits to right;
    }

    hex.chars().rev().collect()
}

fn main() {
    let mut l : LogsManager = LogsManager::new("convertisseur_base_2", true); 
    l.add_log(log_entry::create_log(None, "Démarrage du convertisseur")); 

    let user_input : usize = console::lire_int_unsigned_msg("Entier ? "); // Ask user an input.
    
    let result = convertisseur_base_2(user_input); // Calculate the representation of the number in binary stocked in a Vec<u8>.
    let display_result : String = result.iter() // Transform Vec<u8> to String.
        .map(|bit| if *bit == 1 { '1' } else { '0' })
        .collect();

    let s : String = format!("Décimal : {}\n\t\tBinaire : {}\n\t\tHexadécimal : 0x{}", user_input, display_result, convertisseur_base_16(user_input)); // Create the final output String.
    l.add_log(log_entry::create_log(Some(LogType::Success), s)); // Save & Print the output.
}