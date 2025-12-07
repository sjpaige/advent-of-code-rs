use std::{fs, process};

pub fn read_input_file_from(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(val) => val.lines().map(String::from).collect(),
        Err(e) => {
            eprintln!("Cannot open file: {e}");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {}
