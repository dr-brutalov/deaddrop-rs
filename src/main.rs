use clap::{arg, command, value_parser};
use new::new_user;
use read::read_messages;
use send::send_message;

pub mod session;
pub mod db;
pub mod new;
pub mod read;
pub mod send;
pub mod cocoon;
pub mod hasher;

fn main() {
    // Drop the `logger.rs`, then just use the macro calls to info, warn, etc.
    // Load the `log4rs.yaml` config file. Handle the error during the unwrap!
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or_else(|error| println!("Error while loading `log4rs` config: {}", error)); 

    let args = command!()
        .arg(
            arg!(--to <USER> "the username to send data to")
                .required(false)
                .value_parser(value_parser!(String))
        )
        .arg(
            arg!(--user <USER> "the username to retrieve data for")
                .required(false)
                .value_parser(value_parser!(String))
        )
        .arg(
            arg!(--new "run the utility in add user mode")
                .takes_value(false)
        )
        .arg(
            arg!(--read "run the utility in read mode")
                .takes_value(false)
        )
        .arg(
            arg!(--send "run the utility in send mode")
                .takes_value(false)
        )
        .get_matches();

    let new = args.is_present("new");
    let read = args.is_present("read");
    let send = args.is_present("send");
    let mut user: String = String::new();

    let mut to: String = String::new();

    if let Some(name) = args.get_one::<String>("to") {
        to = to + name;
    } if let Some(name) = args.get_one::<String>("user") {
        user = user + name; // to user and user equals
    }

    if !read && !send && !new {
        println!("Please specify a verb for the utility.");
        println!("Valid verbs: send, read, new");
        return;
    }

    if new && read || new && send || read && send || read && send && new {
        println!("Deaddrop must only use a single verb");
        return;
    }

    if read {
        read_messages(user);
    } else if send {
        send_message(to, user);
    } else if new {
        new_user(user);
    }

}