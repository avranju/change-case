extern crate regex;

use std::io::{self, BufRead};

use regex::{Captures, Regex};

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"[A-Z]").expect("Parsing regex failed");
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut sep = "";
        let output = re.replace_all(&line, |captures: &Captures| {
            let result = captures
                .get(0)
                .map(|m| format!("{}{}", sep, m.as_str().to_lowercase()))
                .unwrap_or_else(|| "".to_string());
            sep = "_";
            result
        });
        println!("{}", output);
    }
}
