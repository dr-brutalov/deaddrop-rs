use cocoon::{Cocoon, Error};
use std::{File, Write, read, remove_file};

// Usage: On first use, the default `dd.db` file
// as the input. The initial function accepts
// a filepath as a string and a bool for determining
// if the target file should be deleted. When initializing
// the program, it would be appropriate to delete the file.
// This should be attached to the logic for generating the
// initial database (post user creation).
pub fn encrypt_data(filepath: str, delete: bool) {
    let data = read(filepath).unwrap();
    let mut file = File::create("dd-enc.db").unwrap_or_else(|error| println!("Error while creating file to encrypt: {}", error));

    // Create a cocoon, the container we will encrypt
    // For the initial run, a password is used.
    // This has obvious problems, but serves as a
    // proof of concept.
    // Future implementations should utilize a file
    // that users would need to provide.
    let cocoon = Cocoon::new(b"password");

    cocoon.dump(data, &mut file).unwrap_or_else(|error| println!("Error while encrypting file: {}", error));

    if delete == true {
        remove_file(filepath)
    }

}


// Usage: Take the filepath, as a string, and decrypt the
// file. A consequence of this method is the generation of
// a temp file that can be operated on. Utilize the delete
// method included with the `encrypt_data` to clean up
// this potential data leakage.
pub fn decrypt_data(filepath: str) {
    let mut file = File::open(filepath).unwrap();
    let cocoon = Cocoon::new(b"password");

    let data = cocoon.parse(&mut file).unwrap_or_else(|error| println!("Error while decrypting file: {}", error));
    // Create a temp file that is named consistently with
    // the original implementation. This should aid in
    // integration.
    let mut temp_file = File::create("db.db").unwrap_or_else(|error| println!("Error creating temp file: {}", error));

    // Write to the file!
    temp_file.write(&data).unwrap_or_else(|error| println!("Error while writing temp file: {}", error));
}

