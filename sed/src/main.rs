use std::env;
use std::process::exit;

extern crate sed;

use sed::{Sed, distribute_args, get_content_from_file, stream_editor, write_content_to_file};


// 1. read the input from the command line args
// 2. handle all possible erors from there
// 3. extract everything and open the file
// 4. implement sed function with the test from the opened file
// 5. write the resulting text into initial file

fn main() {
    let args: Vec<String> = env::args().collect();
    let (pattern, file) = match distribute_args(args) { // moving ownership to this function
        Ok((pattern, file)) => (pattern, file),
        Err(e) => {
            eprint!("Error: {}", e);
            exit(1);
        },
    };

    let sed_struct = match Sed::new(pattern, file.clone()) {
        Ok(sed) => {
            sed
        },
        Err(e) => {
            eprint!("Error: {}", e);
            exit(1);
        },
    };
    
    let content = match get_content_from_file(&sed_struct.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(1);
        },
    };

    let edited_content = stream_editor(&content, &sed_struct);
    println!("Initial content of the file:\n{}", edited_content);

    if let Err(e) = write_content_to_file(&file, &edited_content) {
        eprintln!("Error writing to file: {}", e);
        exit(1);
    }

}
