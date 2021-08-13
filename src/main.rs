mod constants;
mod explorer; // explore directories

mod create; // create subcommand
mod interpreter; // interpret py files

use std::option::Option::{Some, None};
use std::env::{args, Args};
use std::result::Result;
use std::fs;
use std::path::{Path, PathBuf};

use explorer::Explorer;

fn main() {
    let mut args = args();
    args.next();
    match args.next() {
        Some(val) => {
            match val.as_str() {
                "create" => {
                    // Create project
                    cmd_create(&mut args);
                },
                "clean" => {
                    // Clean directory (cache, build, etc.)
                    cmd_clean(&mut args);
                },
                "build" => {
                    // Build requirements
                    cmd_build(&mut args);
                }

                _ => {println!("Unknown subcommand {}", val)}
            }
        },
        None => {println!("No arguments provided");}
    }
}

/// Subcommand to build certain files
/// (requirements.txt, etc.)
fn cmd_build<'a>(_args: &mut Args)  {
    fn is_py(x: &PathBuf) -> bool {
        match x.extension() {
            Some(ext) => {
                match ext.to_str().unwrap() {
                    "py" => true,
                    _ => false,
                }
            },
            None => false,
        }
    }

    // Make sure the env is valid
    match validate_env() {
        Ok(_) => {},
        Err(x) => {
            println!("Current working directory is not valid: {}", x);
            return;
        }
    };

    // Get all python files
    let mut explorer = Explorer::new(10u32);
    let mut path = PathBuf::new();
    path.push("src");

    explorer.explore(&path, &is_py, 0u32).unwrap();

    // Get all modules in each file
    let mut modules: Vec<String> = Vec::new();

    for file in explorer.results {
        // Read through all lines and find import statements
        //let parser = ;

        for line in interpreter::Parser::new(&Path::new(&file)).lines() {
            let unwrapped = &line.unwrap();

            if interpreter::Parser::is_import(unwrapped) {
                // Find the module
                match interpreter::Parser::get_import_module(unwrapped) {
                    Some(mut m) => {modules.append(&mut m);},
                    None => {},
                }
            }
        }
    }
    
    // Get versions for each module
    let versions = interpreter::Parser::with_versions(&modules);

    // Build final requirements.txt string
    let mut requirements_str = String::new();

    for (module, version) in versions.iter() {
        requirements_str.push_str(
            &format!("{}=={}", module, version)
        );
    }

    match fs::write("requirements.txt", requirements_str) {
        Ok(_) => {println!("Successfully updated requirements.txt");},
        Err(e) => {println!("Error while writing to requirements.txt: {}", e);}
    };
}

/// Subcommand to clean cache and stuff
fn cmd_clean(_args: &mut Args) {
    /// Check if a file/dir is removable
    fn is_removable(x: &PathBuf) -> bool {
        let x = x
            .file_name().unwrap()
            .to_str().unwrap();
    
        match x {
            "__pycache__" | "build" => true,
            _ => false,
        }
    }

    // Make sure the env is valid
    match validate_env() {
        Ok(_) => {},
        Err(x) => {
            println!("Current working directory is not valid: {}", x);
            return;
        }
    };

    // Walk through the directory and remove cache directories
    let mut explorer: Explorer = Explorer::new(10u32);
    
    let mut path = PathBuf::new();
    path.push("src");

    explorer.explore(
        &path,
        &is_removable,
        0u32,
    ).unwrap();

    for dir in &explorer.results {
        fs::remove_dir_all(dir).unwrap();
    }

    // Possibly remove unused dependencies
}

/// Subcommand to create a new project
fn cmd_create(args: &mut Args) {
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

fn validate_env() -> Result<(), &'static str> {
    match std::path::Path::new("src").exists() {
        true => Ok(()),
        false => Err("./src does not exist")
    }
}