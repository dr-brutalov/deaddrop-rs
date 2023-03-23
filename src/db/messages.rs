use super::db::connect;
use crate::hasher::message_hash;
use crate::cocoon::encrypt_data;

pub fn get_messages_for_user(user: String) -> Vec<String> {
    let db = connect();

    let query = "SELECT (data) FROM Messages WHERE recipient = (SELECT id FROM Users WHERE user = :user);";
    let mut stmt = db.prepare(query).expect("expected to prepare query");
    let mut rows = stmt.query(&[(":user", &user)]).expect("expected query to succeed");

    let mut messages = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        messages.push(row.get(0).expect("expected a value in the row"));
    }
    encrypt_data();
    messages
}

pub fn set_message_hash(message: String) -> String {
    message_hash(message)
}

pub fn save_message(message: String, sender: String, recipient: String) {
    let db = connect();

    let hashed_message = message_hash(message.clone());

    let query = "INSERT INTO Messages (sender, recipient, hashed_message, data) VALUES ((SELECT id FROM Users WHERE user = :sender), (SELECT id FROM Users WHERE user = :recipient), :hashed_message, :message);";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");
    stmt.execute(&[(":sender", &sender), (":recipient", &recipient), (":hashed_message", &hashed_message), (":message", &message)]).expect("expected query to execute");
    encrypt_data();
}