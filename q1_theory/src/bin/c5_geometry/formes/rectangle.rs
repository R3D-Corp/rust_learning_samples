

/// Calculate the perimeter of a rectangle by a given radius
/// 
/// # Parameters
/// * `radius` : the wanted rectangle radius
/// 
/// # Return
/// * `f64` : the specified rectangle's perimeter
/// 
/// # Author
/// R3D
pub fn perimeter(height : f64, width : f64) -> Result<f64, &'static str> {
    if width <= 0.0 || height <= 0.0 {
        return Err("Width and height can't be 0 or inferior")
    }

    Ok(2.0 * (height + width))
}

/// Calculate the area of a rectangle by a given radius
/// 
/// # Parameters
/// * `radius` : the wanted rectangle radius
/// 
/// # Return
/// * `f64` : the specified rectangle's area
/// 
/// # Author
/// R3D
pub fn area(height : f64, width : f64) -> Result<f64, &'static str> {
    if width <= 0.0 || height <= 0.0 {
        return Err("Width and height can't be 0 or inferior")
    }

    Ok(height * width)
}
