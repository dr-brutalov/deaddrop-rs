use log::{warn};

use super::db::connect;
use crate::hasher::message_hash;
//use crate::cocoon::encrypt_data;

pub fn get_messages_for_user(user: String) -> Vec<String> {
    let db = connect();

    let query = "SELECT sender, data, hashed_message FROM Messages WHERE recipient = (SELECT id FROM Users WHERE user = :user);";
    let mut stmt = db.prepare(query).expect("expected to prepare query");
    let mut rows = stmt.query(&[(":user", &user)]).expect("expected query to succeed");

    let mut messages = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let sender_id: i32 = row.get(0).expect("expected a user in the row.");
        let message: String = row.get(1).expect("expected a message in the row");
        let stored_message_hash: String = row.get(2).expect("expected a hash of the message in the row");

        // Pull the sender name from the database, thanks, Stone!
        let sender_query ="SELECT user FROM Users WHERE id = :sender_id";
        let mut sender_statement = db.prepare(sender_query).expect("expected to prepare query");
        let mut sender_rows = sender_statement.query(&[(":sender_id", &sender_id)]).expect("expected query to succeed");
        let mut sender = String::new();
        // Magic while loop, drop this and things break. 
        while let Some(send_row) = sender_rows.next().unwrap() {
            sender = send_row.get(0).expect("Expected a send_user value");
        }

        let check_message_hash = message_hash(message.clone());

        let mut val_check = "Hello,".to_string();
        if check_message_hash != stored_message_hash {
            val_check = "MESSAGE HAS BEEN ALTERED:".to_string();
            warn!("At least one message for user: {} have been modified.", user);
            warn!("Application integrity has been breached.")
        }

        let full_message = format!("{} message from user {}: {}", val_check, sender, message);
        
        messages.push(full_message);
    }
    //encrypt_data();
    messages
}

// A public interface to the private function used to generate message hashes
pub fn set_message_hash(message: String) -> String {
    message_hash(message)
    
}

pub fn save_message(message: String, sender: String, recipient: String) {
    let db = connect();

    let hashed_message = set_message_hash(message.clone());

    let query = "INSERT INTO Messages (sender, recipient, hashed_message, data) VALUES ((SELECT id FROM Users WHERE user = :sender), (SELECT id FROM Users WHERE user = :recipient), :hashed_message, :message);";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");
    stmt.execute(&[(":sender", &sender), (":recipient", &recipient), (":hashed_message", &hashed_message), (":message", &message)]).expect("expected query to execute");
    //encrypt_data();
}