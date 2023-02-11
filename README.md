# deaddrop-rs

A deaddrop utility written in Rust. Put files in a database behind a password to be retrieved at a later date.

This is a part of the University of Wyoming's Secure Software Design Course (Spring 2023). This is the base repository to be forked and updated for various assignments. Alternative language versions are available in:
- [Javascript](https://github.com/andey-robins/deaddrop-js)
- [Go](https://github.com/andey-robins/deaddrop-go)

## Versioning

`deaddrop-rs` is built with:
- cargo 1.66.0 (d65d197ad 2022-11-15)
- rust edition 2021

## Usage

`cargo run -- --help` for instructions

Then run `cargo run -- --new --user <username here>` and you will be prompted to create the initial password.

## Database

Data gets stored into the local database file dd.db. This file will not by synched to git repos. Delete this file if you don't set up a user properly on the first go.

## Logging Strategy

To ensure this tool functions as intended, logging needed to be implemented. Exploring [crates.io](https://crates.io/crates/log) brought to light several interesting options. For my first attempt, I've elected to use [simple_logging](https://docs.rs/simple-logging/2.0.2/simple_logging/). The initial minimal testing creates an appropriate file, however, it doesn't write anything useful, yet. Further exploration is needed. This approach was ultimately abandoned in favor of [log4rs](https://docs.rs/log4rs/latest/log4rs/). `log4rs` Requires more configuration, however, my initial experience has been smoother. I found this [medium article](https://medium.com/@nikmas_dev/advanced-logging-in-rust-with-log4rs-2d712bb322de) helpful to get some footing with `log4rs`. After some iteration, it was determined that logging date-time information, a level of reporting, and a useful message. The configuration file is [log4rs.yaml](log4rs.yaml) and the necessary logging function is defined in [logger.rs](src/logger.rs). There are three classes of logging:

* `INFO:` General information, included a log of user behavior. This includes logging who is creating new users, who a message was sent to, and which user checked their messages.
* `WARN:` Currently, the only warning generated is when authentication of a user fails. The purpose of this is to inform on possible brute-force attacks on user passwords. This behavior could later be expanded to track other, similar portions of the attack surface.
* `ERROR`: Tracks behavior that we expect to cause errors within the codebase, such as sending/reading for an unknown user. This could be expanded further if more logging is desired.
The rational behind logging these specific things was both to be as minimal as possible and also allow for auditing of normal use. One downside to logging withing my initial assumptions is the leakage of usage data. This could easily be mitigated by encrypting the logging file, similar to how database security is handled now.

## Mitigation

**NOTICE:** There are currently performance issues. If the program appears to be "hanging", give it a bit. With all the file I/O introduced I've probably borked something. However, everything still works as intended. The logging is also a bit wonky at times, if a `Error while loading `log4rs` config: attempted to set a logger after the logging system was already initialized` is thrown during a run don't stress, logging is still functional and the log file is intact. Further testing would be needed to fix this in general as it isn't a consistent error, although it did start when the `cocoon` processes were introduced. Perhaps this is a symptom of an `I/O` race condition that hasn't impacted usage yet?

My proposed mitigation is to implement a method for protecting the exposed database. The initial method being explored involves using the [cocoon](https://docs.rs/cocoon/latest/cocoon/) crate. Similar to how the logging was implemented, a module was built out containing the basic functionality needed. The core of this action depends on calling `encrypt_data` and `decrypt_data` in the correct sequence from [cocoon.rs](src/cocoon.rs). In the case of `encrypt_data`, an unencrypted temporary copy of the database needs to already exist (assumed to be named "dd.db"). The file is then encrypted and the temporary file is deleted. The `decrypt_data` function simply decrypts the encrypted database, 'dd-enc.db' and creates an unencrypted copy, `dd.db`. To utilize both functions, and secure container, called a `cocoon` in this context, is used. The initial proof of concept used a hard-coded password to encrypt the cocoon. This is obviously neither ideal, nor very secure. 

To improve this, an additional mechanism was be implemented. Users will need to provide a file,`secret.txt`, containing a case-sensitive password. Since this tool would most likely be distributed as an executable binary file, this requirement would need to be shared exterior to the tool. Users would be expected to provide and remove the `secret.txt` file when using the tool. This is not a perfect solution either, further improvement could be made to require a user to utilize a USB-based storage device containing the required file, allowing for obfuscation from writing to internal memory. For a proof of concept, an example secret file is included. A full release of this tool would not include this file to encourage individualized usage.

The `secret.txt` file must be provided to initialize the program. This has been tested with a single line of input in said file, anything more fancy may not function. Otherwise, a `tread 'main' panicked at 'called \`Result::unwrap()\` on \`Err\` value: Cryptography', src/cocoon.rs:49:40` error is thrown. This intentionally left ambiguous. Without access to the original code, an attacker will have a difficult time diagnosing the issue.
