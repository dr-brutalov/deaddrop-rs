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

## MAC Implementation

A message authentication code (MAC) has been added to address the issue of message integrity. This was done via the [hasher](src/hasher.rs) module, a simple SHA2 hashing function. Upon a message being created, a hash of that data is calculated and stored in the message database. Upon reading the messages for a user, the hash of each message is verified. If a message has been tampered with, `"MESSAGE HAS BEEN ALTERED:"` is prepended. This is only applied to each altered message, allowing a user to discern which, if any messages have been altered. 

One might observe that simply storing the hash is an insufficient method for establishing a MAC; what's to prevent an attacker from altering the message **and** the hash? To address this very reasonable concern, the database includes the use of a `TRIGGER` that prevents direct editing the column that holds the message hash. To test this is working as intended, follow the procedure below. 

1. Remove the usage of **cocoon** throughout the `deaddrop-rs`. Easiest way to do this is comment out `pub mod cocoon;` in [main.rs](src/main.rs) and then resolve the subsequent errors.
2. In [db.rs](src/db/db.rs), replace `"dd-enc.db"` in the `connect()` function with `"dd.db"`.
3. View the database contents and identify the ID of a message that you wish to alter for testing purposes.
4. Using your sql library of choice, run the SQL query given below. I used `sqlite3` for my testing and validation so milage may vary slightly. 
```sql
UPDATE Messages
SET data = "New message content goes here."
WHERE id =1;
```
5. Upon using the application to read the edited message, it should be flagged as outlined above.

Using the procedure outlined above, one can easily verify that the built-in protection for the message hash works as intended, simply alter the SQL query to change the contents of `hashed_message`.

After verifying the MAC and extra database security, ensure that the cocoon features are reenabled. Once could also test this behavior in a more integrated way, however, this method of testing ensures that the the new, individual addition works as intended and not as a side effect of the underlying database encryption schema.

Using the same technique that was used to secure the message hash, the sender of a message is also now also tracked. To further the usability and transparency of the application, the sender of each message is listed when messages are checked. To ensure that the messaging system isn't abused, users are now required to authenticate prior to sending a message. As an example, if user `bar` wants to send to user `foo` the command below is used. 
```bash
cargo run -- --send --to foo --user bar
``` 