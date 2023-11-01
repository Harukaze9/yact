use serde_yaml::{Value};
use super::command_extractor;
use super::complition_command_extractor;
use super::special_operation_processor;
use super::special_operation;
use super::utils;

pub enum Mode {
    Execution,
    Completion,
}

#[derive(PartialEq, Debug)]
pub enum OperationStatus {
    Command,
    CompletionCommand,
    CompletionText,
    CompletionDefault,
    SpecialOperation,
    // SpecialOperationClipboard,
    IncompleteArgs,
    NoExecCommands,
    Error,
}

impl ToString for OperationStatus {
    fn to_string(&self) -> String {
        match self {
            OperationStatus::Command => "command",
            OperationStatus::CompletionCommand => "completion_command",
            OperationStatus::CompletionText => "completion_text",
            OperationStatus::CompletionDefault => "completion_default",
            OperationStatus::SpecialOperation => "special_operation",
            // OperationStatus::SpecialOperationClipboard => "special_operation_clipboard",

            OperationStatus::IncompleteArgs => "incomplete_args",
            OperationStatus::NoExecCommands => "no_exec_commands",

            OperationStatus::Error => "error",
        }.to_string()
    }
}

pub struct OperationResult {
    pub status: OperationStatus,
    pub result: String,
    pub message: String,
}

pub fn exec_operation(op: Mode, yaml_path: &str, params: &[&str]) -> OperationResult {
    match load_yaml(yaml_path) {
        Ok(doc) => {
            let root_name = utils::extract_filename_from_path(yaml_path);
            match op {
                Mode::Execution => {
                    match special_operation::detect_special_operation(params) {
                        Ok(Some((operation, path, args))) => special_operation_processor::operate_command_on_yaml(doc, yaml_path.to_string(), operation, path, args),
                        Ok(None) => command_extractor::extract_command_from_yaml(&doc, params, root_name),
                        Err(err_msg) => OperationResult { status: OperationStatus::Error, result: "".to_string(), message: err_msg.to_string() },
                    }
                },
                Mode::Completion => complition_command_extractor::extract_completion_command_from_yaml(&doc, params, root_name),
            }
        },
        Err(err_msg) => OperationResult { status: OperationStatus::Error, result: "".to_string(), message: err_msg },
    }
}


pub fn load_yaml(yaml_path: &str) -> Result<Value, String> {
    let file_content = std::fs::read_to_string(&yaml_path);
    if let Err(e) = file_content {
        return Err(format!("failed to read file '{}': {}", yaml_path, e));
    }

    let content = file_content.unwrap();

    // check file is empty or not
    if content.trim().is_empty() {
        return Ok(Value::Mapping(serde_yaml::Mapping::new()));
    }

    let doc: Result<Value, _> = serde_yaml::from_str(&content);
    if let Err(e) = doc {
        return Err(format!("error parsing YAML from '{}': {}", yaml_path, e));
    }

    Ok(doc.unwrap())
}
