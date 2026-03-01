use helmo_b1_rust::{logger::{log_entry, logs_manager::LogsManager}, tools::console};

// Cosntant value defined for example.
const BASE_PRICE : f32 = 25.80;
const REDUCTION : f32 = 0.15;

fn main() {
    let mut l = LogsManager::new("c6_ticket_sncb", true);
    let age = console::lire_type::<u8>(Some("Votre âge ? "));
    
    // Calculate the reduction percentage.
    let reduc = 1.0 - match age {
        0..=11 => 1.0,
        12..=25 => REDUCTION,
        65.. => REDUCTION,
        _ => 0.0
    };
    let price = BASE_PRICE * reduc;

    let message = format!("Prix à payer {:.2}", price);
    l.add_log(log_entry::create_log(None, message));
}