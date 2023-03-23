use log::{error, warn, info};

use crate::{session, db::users};
use std::io::{self, BufRead};

pub fn new_user(user: String) {
    let user_exists = users::get_user(user.clone()).is_some();

    if !users::no_users() && !user_exists {
        error!("Failed to find user: {}", {user});
        
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        // With the inclusion of error handling here, we need to be able to reference
        // `user`. To accommodate this, the .clone() method was needed.
        warn!("Authentication failed for user: {}", user);
        panic!("Unable to authenticate user");
    }

    println!("Username: ");
    let new_user = get_new_username();
    let new_pass_hash = session::get_password();

    users::set_user_pass_hash(new_user.clone(), new_pass_hash);
    info!("User {} created a new user: {}", user, new_user);
}

fn get_new_username() -> String {
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}