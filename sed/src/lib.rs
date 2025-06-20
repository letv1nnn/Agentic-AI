use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sed {
    pub oldstring: String,
    pub newstring: String,
    pub flag: String,
    pub file: String,
}

impl Sed {
    pub fn new(pattern: String, file: String) -> Result<Self, &'static str> {
        let splitted_pattern: Vec<String> = pattern
        .split("|")
        .map(|s| s.to_string())
        .collect();
        match splitted_pattern.len() == 4 {
            true => Ok(Sed {
                oldstring: splitted_pattern[1].clone(),
                newstring: splitted_pattern[2].clone(),
                flag: splitted_pattern[3].clone(),
                file: file.clone(),
            }),
            false => Err(
                "Pattern should have pipes '|' characters as a splitters and should have 4 parts, (s, oldstring, newstring, flag)"
            ),
        }
    }

}


pub fn write_content_to_file(filename: &String, content: &String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn stream_editor(content: &String, sed_struct: &Sed) -> String {
    let mut edited_lines = Vec::new();
    for line in content.lines() {
        let new_line = if line.contains(&sed_struct.oldstring) {
            match sed_struct.flag.as_str() {
                "g" => line.replace(&sed_struct.oldstring, &sed_struct.newstring),
                _ => line.replacen(&sed_struct.oldstring, &sed_struct.newstring, 1),
            }
        } else {
            line.to_string()
        };
        edited_lines.push(new_line);
    }
    edited_lines.join("\n")
}

pub fn get_content_from_file(filename: &String) -> std::io::Result<String> {
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn distribute_args(args: Vec<String>) -> Result<(String, String), &'static str>{
    match args.len() > 2 {
        true => Ok((args[1].clone(), args[2].clone())),
        false => Err(
            "You have to pass at least three arguments. <cargo run> <pattern> <file> [optional_output_file]"
        ),
    }
}
