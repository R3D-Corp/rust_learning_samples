use std::{str::FromStr};
use helmo_b1_rust::{logger::{log_entry, log_type::LogType, logs_manager::LogsManager}, tools::{console, db::OxideDb}};

const CURRENCY : [u8; 6] = [200, 100, 50, 20, 10, 5];

enum Commands {
    Withdraw,
    Deposit,
    Check,
    Exit
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "withdraw" | "retrait" => Ok(Commands::Withdraw),
            "deposit" | "depot" => Ok(Commands::Deposit),
            "check" | "consulter" => Ok(Commands::Check),
            "exit" | "quitter" => Ok(Commands::Exit),
            _ => Err(())
        }
    }
}

fn get_cash(mut amount : u64) -> Vec<u16> {
    let mut cash : Vec<u16> = Vec::with_capacity(CURRENCY.len()); 

    for cur in CURRENCY {
        let cur = cur as u64;

        cash.push((amount / cur) as u16);
        amount = amount % cur;
    }
    cash
}

fn get_amount(user_id: &str, db : &mut OxideDb) -> u64 {
    match db.get::<u64, _>(user_id) {
        Ok(value) => value,
        Err(_) => 0,
    }
}


fn deposit(amount : u64, user_id : &str, db : &mut OxideDb) {
    let previous_amount = get_amount(user_id, db);

    db.put(
        user_id, 
        &(amount + previous_amount)
    ).unwrap();
} 

fn withdraw(amount : u64, user_id : &str, db : &mut OxideDb) -> Option<Vec<u16>> {
    let current_amount = get_amount(user_id, db);

    if current_amount >= amount {
        db.put(
            user_id, 
            &(current_amount - amount)
        ).unwrap();
        return Some(get_cash(amount));
    };
    None
} 


fn main() {
    let mut db = OxideDb::new("c7_distributeur_billets");
    let mut l = LogsManager::new("c7_distributeur_billets", true);
    
    let user_id : u8 = console::lire_type_condition(Some("Qui êtes vous (votre chiffre d'authentification)"), |number| {
        if *number == 0 { return false };
        true
    });
    let user_id : String = format!("user_{}", user_id);
    
    loop {
        let command = console::lire_type::<Commands>(Some("Commande : "));

        match command {
            Commands::Withdraw => {
                let amount = console::lire_type_condition(Some("Montant que vous souhaitez retirer ?"), |value| {
                    let last_type = CURRENCY.last().unwrap_or(&(0 as u8));
                    if *value == 0 || *value % ((*last_type) as u64) != 0 {
                        return false;
                    }
                    true
                });

                if let Some(cash) = withdraw(amount, &user_id, &mut db) {
                    let mut user_cash : String = String::new();
                    for (nb_billets, valeur) in cash.iter().zip(CURRENCY.iter()) {
                        user_cash += &format!("{} billet(s) de {}€\n", nb_billets, valeur);
                    }
                    l.add_log(log_entry::create_log(None, user_cash));
                } else {
                    l.add_log(log_entry::create_log(Some(LogType::Error), "Vous n'avez pas assez d'argent."));
                }
            },
            Commands::Deposit => {
                let amount = console::lire_type(Some("Montant que vous souhaitez déposer ?"));
                deposit(amount, &user_id, &mut db);
                l.add_log(log_entry::create_log(Some(LogType::Success), "Depôt enregistré."));
            },
            Commands::Check => {
                let message = format!("Montant : {}", get_amount(&user_id, &mut db));
                l.add_log(log_entry::create_log(None, message));
            },
            Commands::Exit => break
        };
    }
}