use rand::Rng;
use std::cmp::Ordering;
use std::io;

use helmo_b1_rust::logger::log_entry;
use helmo_b1_rust::logger::log_type::LogType;
use helmo_b1_rust::logger::logs_manager::LogsManager;

use helmo_b1_rust::tools::console;
fn main() {
    let mut l = LogsManager::new("test", true);
    l.add_log(log_entry::create_log_from_text(
        LogType::Info,
        "Démarrage du jeu.",
    ));

    let s = console::lire_string_msg("salut ton nom?");

    println!("{}", s);

    println!("Devinez le nombre !");
    let nombre_secret: u8 = rand::thread_rng().gen_range(1..=100);
    let mut supposition = String::new();

    loop {
        supposition.clear();
        println!("Veuillez entrer un nombre.");

        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entrée utilisateur");

        let supposition: u8 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        println!("Votre nombre : {}", supposition);
        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
    l.add_log(log_entry::create_log_from_text(LogType::Success, "Game finished !"));
}
