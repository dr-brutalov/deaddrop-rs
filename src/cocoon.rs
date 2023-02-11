use cocoon::{Cocoon, Error};
use std::fs::{File, read, remove_file};
use std::io::Write;


// Usage: Revised to drop function inputs, we'll always operate on
// the same file names and should consistently delete the temporary
// unencrypted database after creating an updated, encrypted copy.
// When initializing the program, it would be appropriate to delete 
// the file. This should be attached to the logic for generating the 
// initial database (post user creation).
pub fn encrypt_data() {
    let data = read("dd.db").unwrap();
    let mut file = File::create("dd-enc.db").unwrap();

    let secret = read("secret.txt").unwrap();
    let temp_secret = &secret;
    let secret_bytes: &[u8] = &temp_secret;

    // Create a cocoon, the container we will encrypt
    // For the initial run, a password is used.
    // This has obvious problems, but serves as a
    // proof of concept.
    // Future implementations should utilize a file
    // that users would need to provide.
    let cocoon = Cocoon::new(secret_bytes);

    cocoon.dump(data, &mut file).unwrap_or_else(|error:Error| println!("Error while encrypting file: {:?}", error));

    remove_file("dd.db").unwrap();
}


// Usage: No input is passed needed, the required inputs are static.
// A consequence of this method is the generation of
// a temp file that can be operated on. Utilize the delete
// method included with the `encrypt_data` to clean up
// this potential data leakage.
pub fn decrypt_data() {
    let mut file = File::open("dd-enc.db").unwrap();
    //let cocoon_secret = read("secret.txt").unwrap().parse().unwrap();

    let secret = read("secret.txt").unwrap();
    let temp_secret = &secret;
    let secret_bytes: &[u8] = &temp_secret;

    let cocoon = Cocoon::new(secret_bytes);

    let data = cocoon.parse(&mut file).unwrap();
    // Create a temp file that is named consistently with
    // the original implementation. This should aid in
    // integration.
    let mut temp_file = File::create("dd.db").unwrap();

    // Write to the file!
    temp_file.write(&data).unwrap();
}

// // A function that reads in a file and uses the contents to create a cocoon,
// // the secure container used for file encryption/decryption.
// fn make_cocoon() -> cocoon::Cocoon {
//     let mut file = File::open("secret.txt").to;
//     let cocoon

// }