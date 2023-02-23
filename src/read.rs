use log::{error, warn, info};

use crate::{session, db::{messages, users}};

pub fn read_messages(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        error!("Unknown user {}; No messages available.", user);
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        warn!("Unable to authenticate read message(s) for {}", user);
        panic!("Unable to authenticate user");
    }

    let messages = messages::get_messages_for_user(user.clone());
    for message in messages {
        println!("{:?}", message);
    }
    info!("User {} accessed their messages, yo.", user);
}