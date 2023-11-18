use std::{fs, path::Path, io::Read};

use crate::{logformat, logger::writer::LogStatus};

pub fn handle_help_command(parent: &Path) {
    let mut help_file = match fs::File::open(format!("{}\\..\\etc\\help.txt", parent.display())) {
        Ok(f) => f,
        Err(e) => {
            let error: String = LogStatus::Error.into();
            return eprintln!("{}{e}", logformat!("", error)) 
        }
    };
    let mut string = String::new();
    match help_file.read_to_string(&mut string){
        Ok(_) => (),
        Err(e) => {
            let error: String = LogStatus::Error.into();
            return eprintln!("{}{e}", logformat!("", error)) 
        }
    };
    
    println!("{}", string);
}
