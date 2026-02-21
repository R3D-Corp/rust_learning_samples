use helmo_b1_rust::tools::console;

use crate::formes::{cercle, rectangle};

pub mod formes;


fn display_result(perimeter : f64, aire : f64, form : &str) {
    println!("Le {} a une aire de {:.2}cm² et un perimètre de {:.2}cm", form, aire, perimeter);
}

fn main() {
    let form = console::lire_string_msg("Avec quelle forme voulez vous travailler ? ");

    match form.to_lowercase().as_str() {
        "cercle" => {
            let radius = console::lire_double_msg("Rayon en cm ?");
            display_result(cercle::perimeter(radius), cercle::aire(radius), &form);
        }

        "rectangle" => {
            let width = console::lire_double_msg("Longueur en cm ? ");
            let height = console::lire_double_msg("Hauteur en cm ? ");
            
            match (rectangle::perimeter(height, width), rectangle::aire(height, width)) {
                (Ok(p), Ok(a)) => display_result(p, a, &form),
                (Err(e), _) | (_, Err(e)) => eprintln!("Erreur : {}", e),
            }
        }
        _ => println!("Forme non prise en charge.")
    }
}