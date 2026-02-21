use std::f64::consts::PI;

pub fn perimeter(radius : f64) -> f64 {
    2.0 * PI * radius
}

pub fn aire(radius : f64) -> f64 {
    PI * radius.powi(2)
}