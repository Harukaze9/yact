use colored::*;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let bracket_re = Regex::new(r"\[(.*?)\]").unwrap();
    let dash_re = Regex::new(r"─ (\S+)").unwrap();
    let divider_re = Regex::new(r"^---$").unwrap();

    let mut is_next_line_magenta = false;
    let mut result = String::new();

    for line in buffer.lines() {
        let mut processed_line = line.to_string();

        // Apply green color to text inside "[]"
        processed_line = bracket_re.replace_all(&processed_line, |caps: &regex::Captures| {
            format!("[{}]", caps[1].green().to_string())
        }).to_string();

        // Apply light magenta to strings following "─ ", except when starting with "["
        processed_line = dash_re.replace_all(&processed_line, |caps: &regex::Captures| {
            if caps[1].starts_with('[') {
                format!("─ {}", &caps[1])
            } else {
                // Use truecolor to simulate light magenta
                format!("─ {}", caps[1].truecolor(229, 170, 255).to_string())
            }
        }).to_string();

        if is_next_line_magenta {
            // Use truecolor to simulate light magenta for the entire line
            processed_line = processed_line.as_str().truecolor(229, 170, 255).to_string();
            is_next_line_magenta = false;
        }

        // Detect "---" to mark the next line light magenta
        if divider_re.is_match(line) {
            is_next_line_magenta = true;
        }

        result.push_str(&processed_line);
        result.push('\n');
    }

    print!("{}", result);
}
