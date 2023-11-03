use serde_yaml::{Value};
use super::command_traverser::traverse_yaml_by_params;
use super::extractor_util;
use super::OperationResult;
use super::OperationStatus;
use super::utils;

pub fn extract_command_from_yaml(doc: &Value, params: &[&str], root_name: &str) -> OperationResult {
    let (value, args, _) = traverse_yaml_by_params(doc, params);

    // assume commands are sorted by number of place holders
    let commands = extractor_util::get_commands(&value);
    if !commands.is_empty() {
        let mut last_message = String::new();
        
        for command in &commands {
            let (mut injected_cmd, missing_args_count) = extractor_util::inject_args_into_command(&command, &args);
            injected_cmd = injected_cmd.replace("{self}", &root_name);
            if missing_args_count == 0 {
                return OperationResult {
                    status: OperationStatus::Command,
                    result: injected_cmd,
                    message: "".to_string(),
                };
            }
            last_message = generate_missing_args_message(missing_args_count, &injected_cmd, &args);
        }

        return OperationResult {
            status: OperationStatus::IncompleteArgs,
            result: "".to_string(),
            message: last_message,
        };
    }

    // when no commands are defined
    OperationResult {
        status: OperationStatus::NoExecCommands,
        result: "colorlize".to_string(),
        message: generate_no_command_message(params, &params[..params.len()-args.len()], doc, root_name)
    }
}

fn generate_missing_args_message(missing_args_count: isize, cmd: &str, args: &[&str]) -> String {
    let s = if missing_args_count > 1 {"s"} else {""};
    format!(
        "Error: You need to provide {} more argument{}.\n  Command: \"{}\"\n  Provided args: {:?}",
        missing_args_count, s, cmd, args
    )
}


fn generate_no_command_message(params: &[&str], valid_path: &[&str], doc: &Value, root_name: &str) -> String {
    let (traversed_value, _, parent_key)= traverse_yaml_by_params(doc, params);

    let parent_name = parent_key.unwrap_or_else(|| root_name);
    format!(
        "No command is defined for {}.\nAvailable commands for {} are as follows (use -p to see command definitions).\n{}",
        utils::create_full_path(root_name, params),
        utils::create_full_path(root_name, valid_path),
        utils::generate_tree_string(parent_name, traversed_value, &false)
    )
}