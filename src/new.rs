use crate::{session, db::users};
use std::io::{self, BufRead};

pub fn new_user(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !users::no_users() && !user_exists {
        panic!("User not recognized");
    }

    if !session::authenticate(user).expect("Unable to authenticate user") {
        panic!("Unablee to authenticate user");
    }

    println!("Username: ");
    let new_user = get_new_username();
    let new_pass_hash = session::get_password();

    users::set_user_pass_hash(new_user, new_pass_hash);
}

fn get_new_username() -> String {
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}