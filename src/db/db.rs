use rusqlite::{Connection};
use std::{fs, path::Path};
use crate::logger::log_event;
use crate::cocoon::{decrypt_data, encrypt_data};

pub fn connect() -> Connection {
    let mut must_initialize_db = false;
    if !Path::new("dd.db").exists() {
        must_initialize_db = true;
    }

    let connection = Connection::open("dd.db").unwrap();
    
    if must_initialize_db {
        let query = fs::read_to_string("init.sql").expect("initial schema does not exist");
        let commands = query.split(";\n");

        for command in commands {
            connection.execute(command, ()).unwrap();
        }
        encrypt_data("dd.db", true);
        log_event("info", format!("Database initialized. Not the first logged element? Investigate!"))
    }
    log_event("info", format!("Attempting to decrypt the database..."));
    decrypt_data("dd-enc.db");
    return connection;
}