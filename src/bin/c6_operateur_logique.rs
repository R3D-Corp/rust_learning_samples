use helmo_b1_rust::{logger::{log_entry, logs_manager::LogsManager}, tools::console};

///
/// Function to display with a logManager an unkown x is or not in a range
/// 
/// # Parameters
/// `l` : the LogsManager
/// `range` : the range whe are displaying
/// `is_in` : boolean represent the uknown is or not in.
fn display_info(l : &mut LogsManager, range : u8, is_in : bool ) {
    let cas = match is_in {
        true => "est",
        false => "n'est pas",
    };

    let message : String = format!("x {} un élement de l'intervalle {}", cas, range);
    l.add_log(log_entry::create_log(None, message));
}

fn main() {
    let mut l = LogsManager::new("c6_operateur_logique", true);
    let x = console::lire_int_msg("un réel ? ");

    display_info(&mut l, 1, x >= 8 && x < 10);
    display_info(&mut l, 2, x <= -5 || x >= 5);
    display_info(&mut l, 3, x <= 0 || (x >= 5 && x < 10));
    display_info(&mut l, 4, x != 0 && x >= 5 && x < 10);
}


