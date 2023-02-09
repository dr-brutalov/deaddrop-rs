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

Data gets stored into the local database file dd.db. This file will not by synched to git repos. Delete this file if you don't set up a user properly on the first go

## Mitigation

My proposed mitigation is to implement a method for protecting the exposed database. The initial method being explored involves using the [cocoon](https://docs.rs/cocoon/latest/cocoon/) crate.

## Logging

To ensure this tool functions as intended, logging needed to be implemented. Exploring [crates.io](https://crates.io/crates/log) brought to light several interesting options. For my first attempt, I've elected to use [simple_logging](https://docs.rs/simple-logging/2.0.2/simple_logging/). The initial minimal testing creates an appropriate file, however, it doesn't write anything useful, yet. Further exploration is needed.