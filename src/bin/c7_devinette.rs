use std::cmp::Ordering;

use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::console};
use rand::Rng;

const MAX_GAME : u8 = 5;

fn main() {
    let mut l = LogsManager::new("c7_devinette", true);

    let mut rng = rand::thread_rng();
    let secret_number: u8 = rng.gen_range(1..=10);

    l.add_log(log_entry::create_log(None, "=========="));

    for iteration in 1..=MAX_GAME {
        let entier : u8 = console::lire_type_condition(Some("Entier de 1 à 10"), |value| {
            if *value > 10 || *value == 0 { return false }
            true
        });

        match entier.cmp(&secret_number) {
            Ordering::Less => l.add_log(log_entry::create_log(None, "Plus grand")),
            Ordering::Greater => l.add_log(log_entry::create_log(None, "Plus petit")),
            Ordering::Equal => {
                l.add_log(log_entry::create_log(Some(LogType::Success), "Bravo ! vous avez trouver le nombre mystère."));
                break;
            }
        }

        if iteration == MAX_GAME {
            l.add_log(log_entry::create_log(Some(LogType::Error), "Malheureusement vous n'avez pas trouvé le chiffre."));
            break;
        }
    }
}