use log::info;
use serde_yaml::{Value};
use std::fs::File;
use std::io::Write;
use super::command_traverser::traverse_yaml_by_params;
use super::{SpecialOperationType, OperationResult, OperationStatus, utils, yact_constants};

pub fn operate_command_on_yaml(doc: Value, yaml_path: String, operation_type: SpecialOperationType, keys: &[&str], args: &[&str]) -> OperationResult {
    info!("keys are: [{}], args are] [{}]", keys.join(" "), args.join(" "));
    match operation_type {
        SpecialOperationType::Add => return add_to_yaml(&doc, keys, args, &yaml_path),
        SpecialOperationType::Remove => return remove_from_yaml(&doc, keys, &yaml_path),
        SpecialOperationType::Peak => return peak_yaml(&doc, keys, &yaml_path),
        // SpecialOperationType::Copy => return copy_command_to_clipboard(&doc, keys),
    }
}

fn add_to_yaml(doc: &Value, keys: &[&str], args: &[&str], yaml_path: &str) -> OperationResult {
    if args.is_empty() {
        return OperationResult {
            status: OperationStatus::IncompleteArgs,
            result: "".to_string(),
            message: "The '-a' option requires one argument to specify the command to add.".to_string()
        };
    }

    let mut new_doc = doc.clone();

    let mut target = &mut new_doc;
    for key in keys.iter() {
        if !target.as_mapping().unwrap().contains_key(&Value::String(key.to_string())) {
            target[key] = Value::Mapping(serde_yaml::Mapping::new());
        }
        target = &mut target[key];
    }

    target[yact_constants::KEY_COMMAND] = Value::String(args[0].to_string());

    let yaml_str = serde_yaml::to_string(&new_doc).unwrap();
    let mut file = File::create(yaml_path).unwrap();
    file.write_all(yaml_str.as_bytes()).unwrap();
    let root_name = utils::extract_filename_from_path(yaml_path);

    OperationResult {
        status: OperationStatus::SpecialOperation,
        result: "".to_string(),
        message: format!("YAML Added '{}' to {}", args[0], utils::create_full_path(root_name, keys)).to_string()
    }
}


fn path_does_not_exist_error(root_name: &str, path: &[&str]) -> OperationResult {
    OperationResult {
        status: OperationStatus::Error,
        result: "".to_string(),
        message: format!("The path {} does not exist in the YAML.", utils::create_full_path(root_name, path),),
    }
}


fn remove_from_yaml(doc: &Value, keys: &[&str], yaml_path: &str) -> OperationResult {
    if keys.is_empty() {
        // save as empty yaml
        let mut file = std::fs::File::create(yaml_path).unwrap();
        file.write_all(b"").unwrap();
        
        return OperationResult {
            status: OperationStatus::SpecialOperation,
            result: "".to_string(),
            message: "All content removed as no specific keys were provided.".to_string(),
        };
    }

    let mut new_doc = doc.clone();

    let (parent_keys, remove_key) = keys.split_at(keys.len() - 1);
    let mut target = &mut new_doc;

    // traverse parent_keys
    for key in parent_keys.iter() {
        match target.as_mapping_mut() {
            Some(map) => {
                if let Some(next_target) = map.get_mut(&Value::String(key.to_string())) {
                    target = next_target;
                } else {
                    return path_does_not_exist_error(utils::extract_filename_from_path(yaml_path), keys);
                }
            }
            None => return path_does_not_exist_error(utils::extract_filename_from_path(yaml_path), keys),
        }
    }

    // remove target key
    match target.as_mapping_mut() {
        Some(map) => {
            if map.remove(&Value::String(remove_key[0].to_string())).is_none() {
                return path_does_not_exist_error(utils::extract_filename_from_path(yaml_path), keys);
            }
        }
        None => return path_does_not_exist_error(utils::extract_filename_from_path(yaml_path), keys),
    }

    // save new YAML
    let yaml_str = serde_yaml::to_string(&new_doc).unwrap();
    let mut file = std::fs::File::create(yaml_path).unwrap();
    file.write_all(yaml_str.as_bytes()).unwrap();

    OperationResult {
        status: OperationStatus::SpecialOperation,
        result: "".to_string(),
        message: "".to_string(),
    }
}


fn peak_yaml(doc: &Value, keys: &[&str], yaml_path: &str) -> OperationResult {
    let (value, remains, parent) = traverse_yaml_by_params(&doc, keys);
    let root_name = utils::extract_filename_from_path(yaml_path);
    let msg = format!("Below are command details for {}:\n{}",
                                utils::create_full_path(root_name, &keys[..keys.len()-remains.len()]),
                                utils::generate_tree_string(parent.unwrap_or(root_name), value, &true));

    OperationResult { 
        status: OperationStatus::SpecialOperation, 
        result: "colorlize".to_string(), 
        message: msg,
    }
}

// fn copy_command_to_clipboard(doc: &Value, keys: &[&str]) -> OperationResult {
//     let (value, _, _) = traverse_yaml_by_params(&doc, keys);

//     let commands = extractor_util::get_commands(&value);
//     if let Some(command) = commands.first() {
//         return OperationResult {
//             status: OperationStatus::SpecialOperationClipboard,
//             result: command.to_string(),
//             message: "".to_string(),
//         }
//     }
    
//     OperationResult {
//         status: OperationStatus::Error,
//         result: "".to_string(),
//         message: format!("Cannot found command of [{}]", keys.join(" ")),
//     }
// }
