use crate::{session, db::users, logger::log_event};
use std::io::{self, BufRead};

pub fn new_user(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !users::no_users() && !user_exists {
        log_event("error", format!("Failed to find user: {}", {user}));
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        // With the inclusion of error handling here, we need to be able to reference
        // `user`. To accommodate this, the .clone() method was needed.
        log_event("warn", format!("Authentication failed for user: {}", user));
        panic!("Unable to authenticate user");
    }

    println!("Username: ");
    let new_user = get_new_username();
    let new_pass_hash = session::get_password();

    users::set_user_pass_hash(new_user.clone(), new_pass_hash);
    log_event("info", format!("User {} created a new user: {}", user, new_user))
}

fn get_new_username() -> String {
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}