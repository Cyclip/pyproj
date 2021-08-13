//! Read and interpret Python (.py) files

mod constants;

use std::option::Option::{self, Some, None};

use std::collections::HashMap;
use std::process::Command;
use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::fs::File;

/// Parse python files
pub struct Parser<'a> {
    path: &'a Path,
}

impl<'a> Parser<'a> {
    /// Static method to create a new parser for a single file
    pub fn new(path: &Path) -> Parser {
        Parser {
            path,
        }
    }

    /// Create a line iterator for the file
    pub fn lines(&self) -> io::Lines<io::BufReader<File>> {
        let file = File::open(self.path).unwrap();
        BufReader::new(file).lines()
    }

    /// Static method to identify whether a module is built-in
    pub fn is_built_in(module: &String) -> bool {
        constants::BUILTIN_MODULES
            .iter()
            .any(
                |&x| x == module
            )
    }

    /// Static method to identify whether a line is an import or not
    pub fn is_import(line: &String) -> bool {
        let line_c = line.clone();
        let cleaned = Parser::remove_indent(&line_c);
        let mut iter = cleaned.split(" ");

        return match iter.next() {
            Some("import") | Some("from") => true,
            _ => false,
        };
    }

    /// Static method to get the module(s) from an import line
    pub fn get_import_module(line: &String) -> Option<Vec<String>> {
        let line_c = line.clone();
        let mut iter = line_c.split(" ");

        match iter.next() {
            Some("import") => {
                // import module
                let modules_spaced: String = iter.collect();

                let modules_unspaced = modules_spaced.replace(' ', "");

                let module_iter: Vec<String> = modules_unspaced
                    .split(",")
                    .map(|x| Parser::shorten_module(&x.to_string()))
                    .collect();
                
                return Some(module_iter);
            },
            Some("from") => {
                // from module import attribute
                let module = match iter.next() {
                    Some(v) => String::from(v),
                    None => {return None;},
                };

                let shortened = Parser::shorten_module(&module);
                let vec: Vec<String> = vec![shortened];

                return Some(vec);
            }
            _ => {return None;},
        }
    }                                

    /// Static method to convert a vector of installed modules into a HashMap with the versions respectively
    pub fn with_versions(modules: &Vec<String>) -> HashMap<String, String> {
        let pip_freeze = match Command::new("cmd").args(&["/c", "pip freeze -q -q -q"]).output() {
            Ok(val) => String::from_utf8(val.stdout).unwrap(),
            Err(err) => {
                println!("WARNING: No installed modules found (pip may not be installed on path): {}", err);
                String::new()
            }
        };

        let installed_modules = Parser::format_modules(pip_freeze);
        
        let mut modules_ver: HashMap<String, String> = HashMap::new();

        for m in modules {
            let m_ = &Parser::convert_common_mods(m);
            let ver = match installed_modules.get(m_) {
                Some(v) => v,
                None => {
                    println!("WARNING: Couldn't identify version for module `{}`", m);
                    continue;
                },
            };

            modules_ver.insert(m_.clone(), ver.clone());
        }

        modules_ver
    }

    /// Static method to format raw stdout from pip freeze
    fn format_modules(mods: String) -> HashMap<String, String> {
        let list: Vec<String> = mods
            .split("\r\n")
            .map(|x| x.to_string())
            .collect();
        
        let mut modules: HashMap<String, String> = HashMap::new();

        for module in &list {
            let mut split = module.split("==");
            let mod_name = match split.next() {
                Some(n) => {
                    n.to_string()
                },
                None => {continue;}
            };

            let version = match split.next() {
                Some(v) => v.to_string(),
                None => {continue;}
            };

            modules.insert(mod_name, version);
        }

        modules
    }

    fn convert_common_mods(s: &String) -> String {
        for i in constants::COMMON_MODS {
            if i[0] == s {
                return i[1].to_owned();
            }
        }

        return s.to_owned();
    }

    /// Static method to shorten a module name to its module
    /// i.e "module.attrib" to "module"
    fn shorten_module(module_name: &String) -> String {
        let mut iter = module_name.split(".");

        match iter.next() {
            Some(val) => val.to_string(),
            None => String::new(),
        }
    }

    /// Static method to remove all initial indents
    fn remove_indent(line: &String) -> String {
        let mut new_s = String::new();

        for (i, c) in line.chars().enumerate() {
            match c {
                ' ' | '\t' => {},
                _ => {
                    new_s.push_str(&line[i..]);
                    break;
                }
            }
        }

        new_s
    }
}