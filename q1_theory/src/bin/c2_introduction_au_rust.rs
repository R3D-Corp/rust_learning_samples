use helmo_b1_rust::logger::log_entry;
use helmo_b1_rust::logger::log_type::LogType;
use helmo_b1_rust::logger::logs_manager::LogsManager;
use helmo_b1_rust::tools::console;

fn main() {
    // Create the logger manager
    let mut l : LogsManager = LogsManager::new("introduction_au_rust", true);

    // Add the starting log
    l.add_log(log_entry::create_log(None, "Mon convertisseur"));

    // Get user input & save it
    let lg_en_pouces = console::lire_double_msg("Longueur en pouces ? ");

    // Show the result using the logManager display and save it in log
    l.add_log(log_entry::create_log(Some(LogType::Success), format!("{} cm", lg_en_pouces * 2.54)));
    // Auto-flush of the logger with Drop trait
}