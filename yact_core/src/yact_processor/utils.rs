use serde_yaml::{Value};
use ascii_tree::{Tree, write_tree};

use super::{command_extractor, extractor_util, yact_constants};

pub fn extract_filename_from_path(path: &str) -> &str {
    path.split('/').last().and_then(|filename| {
        filename.strip_suffix(".yaml")
    }).unwrap_or("unknown")
}

/// Create a formatted string for display, like: [root path to subcommand].
pub fn create_full_path(root_name: &str, keys: &[&str]) -> String {
    let mut full_path = vec![root_name];
    full_path.extend_from_slice(keys);
    return format!("[{}]", full_path.join(" "))
}

pub fn generate_tree_string(name: &str, value: &Value, show_detail: &bool) -> String {
    let node = generate_node_recursively(name, value, show_detail);
    let mut output = String::new();
    let _ = write_tree(&mut output, &node);
    return output;
}

static IGNORED_KEYS: &[&str] = &[yact_constants::KEY_COMMAND];

fn generate_node_recursively(name: &str, value: &Value, show_detail: &bool) -> Tree {
    let mut childs: Vec<Tree> = Vec::new();

    if *show_detail {
        let commands = extractor_util::get_commands(value);
        if let Some(cmd) = commands.first() {
            let mut remarks = String::new();
            let mut command_text = cmd.to_string();
            let total_lines_count = command_text.lines().count();
            if total_lines_count > 1 {
                command_text = command_text.lines().next().unwrap_or("").to_string();
                
                let additional_lines_count = total_lines_count - 1;
                let s = if additional_lines_count > 1 { "s" } else { "" };

                remarks.push_str(&format!(" (+{} additional line{} omitted)", additional_lines_count, s));
            }
            if commands.len() > 1 {
                let s = if commands.len() > 2 { "s" } else { "" };
                remarks.push_str(&format!(" (+{} more overload{})", commands.len() - 1, s));
            }
            childs.push(Tree::Node(format!("[{}]{}", command_text, remarks), Vec::new()));
        }
    }
    
    if let Value::Mapping(mapping) = value {
        for (k, v) in mapping.iter() {
            if let Some(key_str) = k.as_str() {
                if IGNORED_KEYS.contains(&key_str) || key_str.chars().all(|c| c.is_numeric() || key_str.starts_with('_')) {
                    continue;
                }
                let node = generate_node_recursively(key_str, v, show_detail);
                childs.push(node);
            }
        }
    }

    let mut help_body = String::new();
    let help_text = if *show_detail {""} else {extractor_util::get_help_text(value)};
    if !help_text.is_empty() {
        let spaces = " ".repeat(16_usize.saturating_sub(name.len()));
        help_body = format!("{}({})", spaces, help_text);
    }

    return Tree::Node(format!("{}{}", name.to_string(), help_body), childs);
}
