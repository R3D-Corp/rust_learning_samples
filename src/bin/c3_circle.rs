use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::console};
use std::f64::consts::PI;

fn main() {
    
    let mut l = LogsManager::new("circle", true);

    l.add_log(log_entry::create_log(None, "Starting chapter3.circle"));

    let radius : f64 = console::lire_double_msg("Rayon ? ");
    let s : String = format!("Aire : {:.2}\nCirconférence : {:.2}", radius.powi(2) * PI, 2.0 * radius * PI);

    l.add_log(log_entry::create_log(Some(LogType::Success), &s));
}