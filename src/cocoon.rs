use cocoon::{Cocoon, Error};
use std::fs::{File, read, remove_file};
use std::io::Write;

// Usage: On first use, the default `dd.db` file
// as the input. The initial function accepts
// a filepath as a string and a bool for determining
// if the target file should be deleted. When initializing
// the program, it would be appropriate to delete the file.
// This should be attached to the logic for generating the
// initial database (post user creation).
pub fn encrypt_data(filepath: &str, delete: bool) {
    let data = read(filepath).unwrap();
    let mut file = File::create("dd-enc.db").unwrap();

    // Create a cocoon, the container we will encrypt
    // For the initial run, a password is used.
    // This has obvious problems, but serves as a
    // proof of concept.
    // Future implementations should utilize a file
    // that users would need to provide.
    let cocoon = Cocoon::new(b"password");

    cocoon.dump(data, &mut file).unwrap_or_else(|error:Error| println!("Error while encrypting file: {:?}", error));

    if delete == true {
        remove_file(filepath).unwrap();
    }

}


// Usage: Take the filepath, as a string, and decrypt the
// file. A consequence of this method is the generation of
// a temp file that can be operated on. Utilize the delete
// method included with the `encrypt_data` to clean up
// this potential data leakage.
pub fn decrypt_data(filepath: &str) {
    let mut file = File::open(filepath).unwrap();
    let cocoon = Cocoon::new(b"password");

    let data = cocoon.parse(&mut file).unwrap();
    // Create a temp file that is named consistently with
    // the original implementation. This should aid in
    // integration.
    let mut temp_file = File::create("dd.db").unwrap();

    // Write to the file!
    temp_file.write(&data).unwrap();
}

