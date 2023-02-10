use std::io::{self, BufRead};

use crate::{db::{users, messages}, logger::log_event};

pub fn send_message(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        log_event("error", format!("Cannot send to an unknown username: {}", user));
        panic!("User not recognized");
    }

    let message = get_user_message();

    messages::save_message(message, user.clone());

    log_event("info", format!("Message sent to user: {}", user));
}

fn get_user_message() -> String {
    println!("Enter your message: ");
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}