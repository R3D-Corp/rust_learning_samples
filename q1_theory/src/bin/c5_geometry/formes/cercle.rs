use std::f64::consts::PI;

/// Calculate the perimeter of a circle by a given radius
/// 
/// # Parameters
/// * `radius` : the wanted circle radius
/// 
/// # Return
/// * `f64` : the specified circle's perimeter
/// 
/// # Author
/// R3D
pub fn perimeter(radius : f64) -> f64 {
    2.0 * PI * radius
}

/// Calculate the area of a circle by a given radius
/// 
/// # Parameters
/// * `radius` : the wanted circle radius
/// 
/// # Return
/// * `f64` : the specified circle's area
/// 
/// # Author
/// R3D
pub fn area(radius : f64) -> f64 {
    PI * radius.powi(2)
}