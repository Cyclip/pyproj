use std::fs;
use std::path::{PathBuf};

pub struct Explorer {
    max_depth: u32,
    pub results: Vec<String>
}

impl Explorer {
    pub fn new(max_depth: u32) -> Explorer {
        Explorer {
            max_depth,
            results: Vec::new()
        }
    }

    pub fn explore(&mut self, path: &PathBuf, predicate: &dyn Fn(&PathBuf) -> bool, depth: u32) -> std::io::Result<()> {
        if path.is_dir() && depth < self.max_depth {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if predicate(&path) {
                    self.results.push(path.display().to_string());
                }

                if path.is_dir() {
                    self.explore(&path, predicate, depth + 1u32).unwrap();
                }
            }
        }

        Ok(())
    }
}