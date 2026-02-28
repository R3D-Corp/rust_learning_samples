use helmo_b1_rust::tools::console;

use crate::formes::{cercle, rectangle};

pub mod formes;

/// 
/// Display the perimeter and the surface of a shape
/// 
/// # Parameters
/// * `perimeter` : The perimeter of the shape
/// * `area` : The area of the shape
/// * `shape` : The shape name
///
/// # Author
/// R3D 
fn display_result(perimeter : f64, area : f64, shape : &str) {
    println!("Le {} a une aire de {:.2}cm² et un perimètre de {:.2}cm", shape, area, perimeter);
}

fn main() {
    let shape = console::lire_string_msg("Avec quelle forme voulez vous travailler ? ");

    match shape.to_lowercase().as_str() {
        "cercle" => {
            let radius = console::lire_double_msg("Rayon en cm ?");
            display_result(cercle::perimeter(radius), cercle::area(radius), &shape);
        }

        "rectangle" => {
            let width = console::lire_double_msg("Longueur en cm ? ");
            let height = console::lire_double_msg("Hauteur en cm ? ");
            
            match (rectangle::perimeter(height, width), rectangle::area(height, width)) {
                (Ok(p), Ok(a)) => display_result(p, a, &shape),
                (Err(e), _) | (_, Err(e)) => eprintln!("Erreur : {}", e),
            }
        }
        _ => println!("Forme non prise en charge.")
    }
}