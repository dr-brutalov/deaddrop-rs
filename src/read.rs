use crate::{session, db::{messages, users}};

pub fn read_messages(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        panic!("Unalbe to authenticate user");
    }

    let messages = messages::get_messages_for_user(user);
    for message in messages {
        println!("{:?}", message);
    }
}