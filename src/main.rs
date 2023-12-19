use std::env;
use std::sync::{Mutex, Arc};
use std::{path::Path, fs};
use chrono::Local;

use crate::generator::generate;
use crate::{arg_parser::args::get_args, core::*, logger::writer::LogWriter};

use arg_parser::args::*;
use logger::writer::LogStatus;

pub mod arg_parser;
pub mod core;
pub mod generator;
pub mod help;
pub mod logger;
pub mod tasks;
pub mod macros;

fn main() {
    // directory generator
    // a cli/command line tool to make a project directory DEMO
    // the project structure can be specified in a .tmplt files

    // TODOS:
    // - Command executor and interpreter module -> done
    // - command line args parser and identifier module -> done
    // - task directory for executing commands asynchronously and concurrently -> done
    // - better error messages and a logger -> done
    // - thats all what ya ass wanting me to be dead

    let exe_dir = env::current_exe().unwrap();
    let parent = exe_dir.parent().unwrap();

    if !Path::new(format!("{}\\..\\logs\\", parent.display()).as_str()).exists() {
        match fs::create_dir(format!("{}\\..\\logs\\", parent.display()).as_str()){
            Ok(_) => (),
            Err(e) => return eprintln!("{}", e) 
        }
    };

    let args = match get_args() {
        Ok(option) => option,
        Err(e) => {
            if e.no_args() {
                return help::handle_help_command(parent);
            }

            let error: String = LogStatus::Error.into();
            return println!("{}{e}", logformat!("", error));
        },
    };

    match args.get_command() {
        CommandLineArgs::New => {
            let f = match args.get_file() {
                Some(f) =>{
                    let info: String = LogStatus::Info.into();
                    println!("{}initializing a new .tmplt file", logformat!("",info));
                    fs::copy(format!("{}\\..\\etc\\default.tmplt", parent.display()), f)
                },
                None => {
                    let warn: String = LogStatus::Warning.into();
                    println!("{}", logformat!("if you would like to initialize a new .tmplt file with any other name dont forget to include the ext as a seconde argument", warn));
                    println!("{}", logformat!("example: tmplt init first_look.tmplt", warn));

                    fs::copy(format!("{}\\..\\etc\\default.tmplt", parent.display()), "new.tmplt")
                }
            };

            match f{
                Ok(_) => {
                    let status: String = LogStatus::Info.into();
                    return println!("{}", logformat!("initialized new template (tmplt) file, for more info consider reading the README.md file in the main repository", status))
                }
                Err(e) => {
                    let error: String = LogStatus::Error.into();
                    return eprintln!("{}{e}", logformat!("", error))
                }
            }
        },
        CommandLineArgs::Generate => {
            let op_logger = if *args.get_save_logs_flag() {
                let now = Local::now();
        
                Some(LogWriter::initialize_logger(format!("{}",now.format("%H:%M:%S"))))
            }else { None };
        
            let logger = match op_logger {
                Some(logger) => Some(Arc::new(Mutex::new(logger))),
                None => None,
            };

            generate(args.clone(), logger.clone());

            match logger {
                Some(logs) => {
                    let logs = logs.lock().unwrap();
        
                    logs.write_to_file()
                },
                None => (),
            }
        },
        CommandLineArgs::Help => help::handle_help_command(parent),
        CommandLineArgs::UnknownArg => {
            return eprintln!("unknown command, use tmplt help to get more info")
        }
    }
}
