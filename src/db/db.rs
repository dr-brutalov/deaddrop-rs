use log::{info, error, warn};
use rusqlite::{Connection};
use core::panic;
use std::{fs, path::Path};
//use crate::cocoon::{decrypt_data, encrypt_data};

pub fn connect() -> Connection {
    let mut must_initialize_db = false;
    if !Path::new("dd.db").exists() {
        must_initialize_db = true;
    }

    let connection = Connection::open("dd.db").unwrap();
    
    if must_initialize_db {
        let query = fs::read_to_string("init.sql").expect("initial schema does not exist");
        let commands = query.split(";\n\n");

        for command in commands {
            connection.execute(command, ()).unwrap_or_else(|error| {
                if error.to_string() == "not an error" {
                    warn!("Still not an error.");
                } else {
                    error!("Database initialization is broken, check the init schema. Error: {}", error);
                    panic!("Database initialization is borked: {}", error);
                }
                0 // C-style return code for all's good!
                });
                
        }
        info!("Database initialized. Not the first logged element? Investigate!");
        //encrypt_data();
    }
    //log_event("info", format!("Attempting to decrypt the database..."));
    //decrypt_data();
    connection
}