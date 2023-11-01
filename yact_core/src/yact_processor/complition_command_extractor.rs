
use serde_yaml::{Value};
use super::extractor_util;
use super::command_traverser;
use super::OperationResult;
use super::OperationStatus;
use super::yact_constants;



/// Extracts a completion command or suggestion from a YAML document based on the provided parameters.
///
/// # Arguments
/// 
/// - `doc`: A reference to the YAML document to traverse.
/// - `params`: An array of parameters used to navigate the YAML document.
/// 
/// # Returns
///
/// An `OperationResult` object containing:
/// 
/// - `status`: The method used for the completion by shell.
/// - `result`: The actual completion content (either a completion command or text).
/// - `message`: Information about available options or further context.
pub fn extract_completion_command_from_yaml(doc: &Value, params: &[&str], root_name: &str) -> OperationResult {
    let (value, args, _) = command_traverser::traverse_yaml_by_params(doc, params);

    let available_options_text = get_available_options_text(&value, &args);

    // Priority 1: Use the specified completion command, if provided.
    if let Some(cmd) = get_completion_command_str(&value, &args) {
        let (mut injected_cmd, _) = extractor_util::inject_args_into_command(&cmd, &args); // TODO: use injection
        injected_cmd = injected_cmd.replace("{self}", &root_name);
        return OperationResult {
            status: OperationStatus::CompletionCommand,
            result: injected_cmd,
            message: available_options_text
        };
    }

    // Priority 2: Attempt to generate completion text based on subcommand names.
    let completion_text = get_completion_text_by_subcommands(&value);
    if !completion_text.is_empty() {
        return OperationResult {
            status: OperationStatus::CompletionText,
            result: completion_text,
            message: available_options_text
        };
    }

    // Priority 3: If the command exists and additional arguments are required, use the default shell completion.
    if extractor_util::get_required_args_count(&value) > args.len() {
        return OperationResult {
            status: OperationStatus::CompletionDefault,
            result: "".to_string(),
            message: available_options_text
        };
    }
    
    // Priority 4: Default case - provide no completions.
    OperationResult {
        status: OperationStatus::CompletionText,
        result: "".to_string(),
        message: available_options_text
    }    
}


fn get_available_options_text(current_value: &Value, args: &Vec<&str>) -> String {
    let mut available_options = vec!["-a"];

    let commands = extractor_util::get_commands(current_value);


    if args.is_empty() {
        available_options.push("-r");
        available_options.push("-p")
    }

    return available_options.join(" ");
}


fn get_completion_text_by_subcommands(value: &Value) -> String {
    if let Value::Mapping(map) = value {
        let params: Vec<String> = map
            .iter()
            .filter_map(|(k, _)| k.as_str())
            .filter(|&key| key != yact_constants::KEY_COMMAND)
            .filter(|&key| !key.starts_with("_"))
            .map(String::from)
            .collect();
        return params.join(" ");
    }
    "".to_string()
}

fn get_completion_command_str(value: &Value, args: &[&str]) -> Option<String> {
    if let Value::Mapping(map) = value {
        let key = Value::Number(serde_yaml::Number::from(args.len()));

        if let Some(val) = map.get(&key) {
            return val.as_str().map(String::from);
        }
    }

    None
}