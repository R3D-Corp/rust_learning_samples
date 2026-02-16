
use std::io;
fn lire_type<T: std::str::FromStr>(message: Option<&str>) -> T {
    // let mut input : String = String::new();
    // loop {
    //     if let Some(msg) = message {
    //         println!("{}", msg);
    //     }
    //     input.clear();

    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Echec de la lecture de l'entrée utilisateur");

    //     if let Ok(response) = input.trim().parse::<T>() {
    //         return response;
    //     }
    // }

    lire_type_with_validation::<T, _>(message, |_| true)
}

pub fn lire_type_with_validation<T: std::str::FromStr, F: Fn(&T) -> bool>(message: Option<&str>, validation : F) -> T {
    let mut input : String = String::new();
    loop {
        if let Some(msg) = message {
            println!("{}", msg);
        }
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture");

        if let Ok(response) = input.trim().parse::<T>() {
            if validation(&response) {
                return response;
            }
        }
    }
}

pub fn lire_string_with_question(message : &str) -> String {
    lire_type::<String>(Some(message))
} 

pub fn lire_string_without_question() -> String {
    lire_type::<String>(None)
} 

pub fn lire_int() -> isize {
    lire_type::<isize>(None)
}

pub fn lire_int_msg(message : &str) -> isize {
    lire_type::<isize>(Some(message))
}

pub fn lire_long() -> i128 {
    lire_type::<i128>(None)
}

pub fn lire_long_msg(message : &str) -> i128 {
    lire_type::<i128>(Some(message))
}

pub fn lire_double() -> f64 {
    lire_type::<f64>(None)
}

pub fn lire_double_msg(message : &str) -> f64 {
    lire_type::<f64>(Some(message))
}