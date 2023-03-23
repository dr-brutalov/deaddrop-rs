use std::io::{self, BufRead};

use log::{info, error, warn};

use crate::{db::{users, messages}, session};

pub fn send_message(to: String, user: String) {
    let user_exists = match users::get_user(to.clone()) {
        Some(_) => true,
        None => false,
    };
    if !user_exists {
            error!("Cannot send to an unknown username: {}", to);
            panic!("User not recognized");
        }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        warn!("Unable to authenticate read message(s) for {}", user);
        panic!("Unable to authenticate user");
    }

    let message = get_user_message();

    messages::save_message(message, user.clone(), to.clone());

    info!("Message sent to user: {} from: {}", to, user);
}

fn get_user_message() -> String {
    println!("Enter your message: ");
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}