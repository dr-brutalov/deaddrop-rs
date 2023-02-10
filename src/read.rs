use crate::{session, db::{messages, users}, logger::log_event};

pub fn read_messages(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        log_event("error", format!("Unknown user {}; No messages available.", user));
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        log_event("warn", format!("Unable to authenticate read message(s) for {}", user));
        panic!("Unable to authenticate user");
    }

    let messages = messages::get_messages_for_user(user.clone());
    for message in messages {
        println!("{:?}", message);
    }
    log_event("info", format!("User {} accessed their messages, yo.", user));
}