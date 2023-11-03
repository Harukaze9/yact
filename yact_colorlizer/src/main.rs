use colored::*;
use regex::Regex;
use std::io::Read;

const MAGENTA_RGB: (u8, u8, u8) = (229, 170, 255);

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let colored_output = colorize_input(&buffer);
    print!("{}", colored_output);
}

fn colorize_input(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = String::new();
    
    let bracket_colorizer = |content: &str, index: usize| -> ColoredString {
        if index < 2 {
            content.truecolor(MAGENTA_RGB.0, MAGENTA_RGB.1, MAGENTA_RGB.2)
        } else {
            content.green()
        }
    };

    let mut is_next_line_magenta = false;

    for (index, &line) in lines.iter().enumerate() {
        let processed_line = if is_next_line_magenta {
            // Color the entire line
            is_next_line_magenta = false; // Reset flag
            color_entire_line(line, MAGENTA_RGB)
        } else {
            // Color parts of the line
            color_line_parts(line, index, &bracket_colorizer)
        };

        // Detect divider line
        if line.trim() == "---" {
            is_next_line_magenta = true;
        }

        result.push_str(&processed_line);
        result.push('\n');
    }

    result
}

fn color_entire_line(line: &str, color: (u8, u8, u8)) -> String {
    line.truecolor(color.0, color.1, color.2).to_string()
}

fn color_line_parts<F>(line: &str, index: usize, bracket_colorizer: F) -> String
where
    F: Fn(&str, usize) -> ColoredString,
{
    let bracket_re = Regex::new(r"\[(.*?)\]").unwrap();
    let dash_re = Regex::new(r"─ (\S+)").unwrap();

    let line = bracket_re.replace_all(line, |caps: &regex::Captures| {
        format!("[{}]", bracket_colorizer(&caps[1], index).to_string())
    }).to_string();

    dash_re.replace_all(&line, |caps: &regex::Captures| {
        if caps[1].starts_with('[') {
            format!("─ {}", &caps[1])
        } else {
            format!("─ {}", caps[1].truecolor(MAGENTA_RGB.0, MAGENTA_RGB.1, MAGENTA_RGB.2).to_string())
        }
    }).to_string()
}
