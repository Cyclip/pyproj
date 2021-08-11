mod constants;
mod create; // create subcommand

use std::option::Option::{Some, None};
use std::env;
use std::result::Result;
use std::fs;

/// Identify whether a string is valid and has no conflicting names
fn validate_name(s: &str) -> bool {
    if s.len() == 0 {
        false
    } else if constants::DISALLOWED_NAMES.iter().any(|x| &s == x) {
        false
    } else {
        true
    }
}

/// Subcommand to create a new project
fn cmd_create(args: &mut std::env::Args) {
    let project_name = match args.next() {
        Some(val) => {
            let val = val.as_str();
            if validate_name(&val) {
                match create::create_project(&val) {
                    Result::Ok(proj_name) => {println!("Successfully created project at ./{}", proj_name);},
                    Result::Err(err) => {
                        println!("Error while creating project: {}", err);
                        // remove dir
                        fs::remove_dir_all(&val).unwrap();
                    }
                }
            } else {
                println!("Invalid project name '{}'", val);
            }
        },
        None => {println!("Missing argument project_name");}
    };
}

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(val) => {
            match val.as_str() {
                "create" => {
                    // Create project
                    cmd_create(&mut args);
                },

                _ => {println!("Unknown subcommand {}", val)}
            }
        },
        None => {println!("No arguments provided");}
    }
}