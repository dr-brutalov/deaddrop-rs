use log::{error, info, warn, debug}; // Gives us levels of logging
use log4rs; // The bread and butter, we'll use a custom config to make things look real nice

mod logger {
    pub fn log_event(category: &str, event: String) {
        // Load the `log4rs.yaml` config file. Handle the error during the unwrap!
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap_or_else(|error| println!("Error while loading `log4rs` config: {}", error)); 

        // Based on the level of logging expected, apply the appropriate macro.
        // These macros come from `log4rs` and its interface with the `log` crate.
        match category {
            "info"  => info!("{event}"),
            "warn"  => warn!("{event}"),
            "error" => error!("{event}"),
            _       => debug!("Invalid category specified while logging: {category}, {event}")
        }
    }
}