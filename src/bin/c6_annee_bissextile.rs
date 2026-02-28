use helmo_b1_rust::tools::console;



///
/// Fuinction to determine if a specified year is leap year.
/// 
/// # Parameters 
/// `year` : the year to check of.
/// 
/// # Return 
/// `bool` : the year is a leap year or not.
fn is_bissextile(year : i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}


fn main() {
    let year = console::lire_type::<i32>(Some("Année ? "));

    let status = match is_bissextile(year) {
        true => "est",
        false => "n'est pas"
    };

    println!("L'année {} {} bissextile", year, status)
}