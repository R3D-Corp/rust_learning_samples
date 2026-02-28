use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::console};

///
/// Function that ask a input to the user. 
/// # Parameters
/// * `maximum` : a u16 which represent the maximum that the user can enter without provacting an reasking.
/// * `message` : the output question showed to the user.
/// 
/// # Return
/// * `u16` : the validate & verified user input.
/// 
/// # Author
/// R3D
fn ask_time(maximum : u16, message : &str) -> u16 {
    let time = console::lire_type_condition::<u16, _>(Some(message), |time: &u16| {
        if *time >= maximum {
            return false;
        }
        true
    });
    time
}  

fn main() {
    let mut l  = LogsManager::new("somme_durees", true);

    l.add_log(log_entry::create_log(None, "test"));

    // Ask user time and save it.
    let h1 = ask_time(9999, "Combien d'heures ? ");
    let m1 = ask_time(60, "combien de minutes ? ");
    let s1 = ask_time(60, "Combien de secondes ? ");

    // Define variables.
    let (h2, m2, s2): (u16, u16, u16) = (3, 38, 45);
    let (h_total, m_total, s_total, s_incr, m_incr): (u16, u16, u16, u16, u16);

    s_total = (s1 + s2) % 60;
    s_incr = (s1 + s2) / 60;
    
    m_total = (m1 + m2 + s_incr) % 60;
    m_incr = (m1 + m2 + s_incr) / 60;
    
    h_total = h1 + h2 + m_incr;

    let s : String = format!("{}h{}min{}sec", h_total, m_total, s_total);
    l.add_log(log_entry::create_log(Some(LogType::Success), &s));
}