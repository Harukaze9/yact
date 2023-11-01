
use serde_yaml::{Value};
use regex::Regex;

use super::yact_constants;


pub fn get_required_args_count(_value: &Value) -> usize {
    let commands = get_commands(_value);
    if !commands.is_empty() {
        return count_unique_placeholders(&commands[0]);// TODO: fix
    }
    return 0;
}

pub fn count_unique_placeholders(s: &str) -> usize {
    let re = Regex::new(r"\{(\d+)\}").unwrap();
    let mut unique_numbers = std::collections::HashSet::new();
    for cap in re.captures_iter(s) {
        if let Some(matched) = cap.get(1) {
            unique_numbers.insert(matched.as_str());
        }
    }
    unique_numbers.len()
}


pub fn inject_args_into_command(cmd: &str, args: &[&str]) -> (String, isize) {
    let mut result = cmd.to_string();

    for (index, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", index);
        result = result.replace(&placeholder, arg);
    }

    let missing_args_count = count_unique_placeholders(&result) as isize;

    (result, missing_args_count)
}


fn get_cmd_value_from_value(value: &Value) -> Option<Vec<String>> {
    // If the value is a string, return a vec containing that string
    if let Some(s) = value.as_str() {
        return Some(vec![s.to_string()]);
    }
    // If the value is a sequence of strings, return the vec of those strings
    else if let Some(array) = value.as_sequence() {
        let strs: Vec<String> = array.iter()
            .filter_map(|v| v.as_str())
            .map(String::from)
            .collect();
        if !strs.is_empty() {
            return Some(strs);
        }
    }
    None
}

pub fn get_commands(value: &Value) -> Vec<String> {
    let mut commands = if let Value::Mapping(map) = value {
        if let Some(cmd) = map.get(&Value::String(yact_constants::KEY_COMMAND.to_string())) {
            get_cmd_value_from_value(cmd).unwrap_or_default()
        } else {
            vec![]
        }
    } else {
        get_cmd_value_from_value(value).unwrap_or_default()
    };

    // Sort commands based on the number of placeholders in descending order
    commands.sort_by(|a, b| count_unique_placeholders(b).cmp(&count_unique_placeholders(a)));

    commands
}

pub fn get_help_text(value: &Value) -> &str {
    if let Value::Mapping(map) = value {
        if let Some(help) = map.get(&Value::String(yact_constants::KEY_HELP.to_string())) {
            if let Some(ret) = help.as_str() {
                return ret;
            }
        }
        
    }
    return "";
}