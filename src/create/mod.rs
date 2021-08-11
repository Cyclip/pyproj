mod file_constants;

use std::process::Command;
use std::result::Result::{self, Ok, Err};
use std::option::Option::{Some, None};
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use std::io::{self, Write};

/// Project configuration
struct Config {
    description: String,
    author: String,
    license: String,
    version: String,
}

/// Create a new project
pub fn create_project<'a>(project_name: &'a str) -> Result<&'a str, &'a str> {
    let dir = Path::new(project_name);

    // Don't overwrite existing project directories
    if dir.exists() {
        // Use if it doesn't contain any files or directories
        if !is_dir_empty(dir) {
            return Result::Err("Directory exists and contains files");
        }
    }

    let cfg = build_config();

    setup_folders(project_name);
    setup_files(project_name);
    setup_files_special(project_name, cfg);

    Result::Ok(project_name)
}

/// Generate Config{}
fn build_config() -> Config {
    let description = input("Project description: ");
    let author = input("Author: ");
    let license = input("License name: ");
    let version = get_py_ver();

    Config {
        description,
        author,
        license,
        version,
    }
}

/// Create folders in a project
fn setup_folders(project_name: &str) {
    let folders = [
        String::from(project_name),
        format!("{proj}/docs", proj=project_name),
        format!("{proj}/examples", proj=project_name),
        format!("{proj}/tests", proj=project_name),
        format!("{proj}/src/{proj}", proj=project_name),
    ];

    for folder in folders.iter() {
        fs::create_dir_all(folder).unwrap();
    }
}

/// Create and write to all files
fn setup_files(project_name: &str) {
    let mut files: HashMap<String, &str> = HashMap::new();

    files.insert(format!("{proj}/.gitignore", proj=project_name), file_constants::FILE_GITIGNORE);
    files.insert(format!("{proj}/MANIFEST.in", proj=project_name), file_constants::FILE_MANIFEST);
    files.insert(format!("{proj}/setup.py", proj=project_name), file_constants::FILE_SETUP_PY);
    files.insert(format!("{proj}/src/{proj}/main.py", proj=project_name), file_constants::FILE_MAIN_PY);
    files.insert(format!("{proj}/tests/test.py", proj=project_name), file_constants::FILE_TEST);

    // Create files with constant content
    for (file, content) in &files {
        match fs::write(file, content) {
            Ok(_x) => {},
            Err(_x) => {
                println!("Couldn't create {}", file);
            }
        };
    }

    // Create empty files
    fs::File::create(format!("{proj}/LICENSE", proj=project_name)).unwrap();
    fs::File::create(format!("{proj}/requirements.txt", proj=project_name)).unwrap();
    fs::File::create(format!("{proj}/tests/__init__.py", proj=project_name)).unwrap();
}

/// Create special files with content based on the project
fn setup_files_special(project_name: &str, cfg: Config) {
    // setup.cfg
    fs::write(
        format!("{proj}/setup.cfg", proj=project_name),
        format!("[metadata]
name = {proj}
version = 0.1.0
license = {license}
description = {desc}
long_description = file: README.md
author = {author}
classifiers=
   Programming Language :: Python :: {version}

[options]
packages = find:
package_dir =
    =src
include_package_data = True
install_requires =
   

[options.packages.find]
where=src
",
            
        proj=project_name, license=cfg.license, desc=cfg.description, author=cfg.author,version=cfg.version)
    ).unwrap();

    fs::write(
        format!("{proj}/README.md", proj=project_name),
        format!("# {proj}

{desc}

## Features

* Feature 1
* Feature 2

## Examples

Here are a few examples:

* [Example 1](https://www.example.com)
* [Example 2](https://www.example.com)


## Usage

### Prequisites
You must have Python 3 installed.
{proj} is usable on:

* Python {version}
* (add more versions if it has been tested)

### Installing
Clone this repository via Git:

```
git clone <url>
```

Install the requirements from `./requirements.txt`:

```
python -m pip install -r requirements.txt
```

## Made with
* [Python](http://python.org/)

## Repo structure
```
│   
├─── .gitignore				
├─── LICENSE				
├─── MANIFEST.in			
├─── requirements.txt		
├─── setup.cfg				
├─── setup.py				
│
├─── docs					Documentation detailing how to use {proj}
├─── examples				Example usage
├─── src					Source code for use in this project
│    └───sysa				
│        └─── main.py		Main python file
│
└─── tests					All tests (unit, etc.)
     └─── __init__.py		
```

## Licensing
{proj} is licensed under [{license}](https://www.example.com)

___
Created by [{author}](https:://www.example.com)",
            proj=project_name, license=cfg.license, desc=cfg.description, author=cfg.author,version=cfg.version
        )
    ).unwrap();
}

/// Check whether or not a directory is empty
fn is_dir_empty(path: &Path) -> bool {
    match fs::read_dir(path) {
        Ok(x) => {x.take(1).count() == 0},
        Err(_x) => {false}
    }
}

fn input(prefix: &str) -> String {
    print!("{}", prefix);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line[..(line.len()-2)].to_string()
}

fn get_py_ver() -> String {
    let output = match Command::new("cmd").args(&["/c", "python --version"]).output() {
        Ok(x) => {String::from_utf8(x.stdout).unwrap()},
        Err(x) => {String::from("err")}
    };

    let split = match output.split(" ").nth(1) {
        Some(x) => {x[..x.len()-2].to_string()},
        None => {
            println!("Couldn't find Python installed on path");
            panic!("cant find python");
        }
    };

    split
}