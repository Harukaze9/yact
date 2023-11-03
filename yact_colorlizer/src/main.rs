use std::io::Read;
use colored::*;
use regex::Regex;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let colored_output = colorize_lines(&buffer);
    print!("{}", colored_output);
}

fn colorize_lines(input: &str) -> String {
    let bracket_re = Regex::new(r"\[(.*?)\]").unwrap();
    let dash_re = Regex::new(r"─ (\S+)").unwrap();
    let divider_re = Regex::new(r"^---$").unwrap();

    let mut is_next_line_magenta = false;
    let mut result = String::new();

    for (index, line) in input.lines().enumerate() {
        let mut processed_line = line.to_string();

        // Detect "---" to mark the next line with truecolor
        if divider_re.is_match(line) {
            is_next_line_magenta = true;
        } else if is_next_line_magenta {
            // Apply truecolor to the specific line after "---"
            processed_line = processed_line.truecolor(229, 170, 255).to_string();
            is_next_line_magenta = false;
        }

        // For the first two lines, use truecolor(229, 170, 255) for text inside "[]"
        if index < 2 {
            processed_line = bracket_re.replace_all(&processed_line, |caps: &regex::Captures| {
                format!("[{}]", caps[1].truecolor(229, 170, 255).to_string())
            }).to_string();
        } else {
            // Apply green color to text inside "[]", except for the line after "---"
            processed_line = bracket_re.replace_all(&processed_line, |caps: &regex::Captures| {
                format!("[{}]", caps[1].green().to_string())
            }).to_string();
        }

        // Apply custom magenta to strings following "─ ", except when starting with "["
        processed_line = dash_re.replace_all(&processed_line, |caps: &regex::Captures| {
            if caps[1].starts_with('[') {
                format!("─ {}", &caps[1])
            } else {
                format!("─ {}", caps[1].truecolor(229, 170, 255).to_string())
            }
        }).to_string();

        result.push_str(&processed_line);
        result.push('\n');
    }

    result
}
