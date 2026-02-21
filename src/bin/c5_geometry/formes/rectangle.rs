pub fn perimeter(height : f64, width : f64) -> Result<f64, &'static str> {
    if width <= 0.0 || height <= 0.0 {
        return Err("Width and height can't be 0 or inferior")
    }

    Ok(2.0 * (height + width))
}


pub fn aire(height : f64, width : f64) -> Result<f64, &'static str> {
    if width <= 0.0 || height <= 0.0 {
        return Err("Width and height can't be 0 or inferior")
    }

    Ok(height * width)
}
