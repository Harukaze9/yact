use serde_yaml::{Value};

pub fn traverse_yaml_by_params<'a>(
    doc: &'a Value, 
    params: &[&'a str]
) -> (&'a Value, Vec<&'a str>, Option<&'a str>) {
    let mut current_value = doc;
    let mut remaining_params = Vec::new();
    let mut parent_key: Option<&'a str> = None;

    for key in params.iter() {
        match current_value {
            Value::Mapping(map) => {
                if let Some(val) = map.get(&Value::String(key.to_string())) {
                    parent_key = Some(key);
                    current_value = val;
                } else {
                    remaining_params.push(*key);
                }
            }
            _ => {
                remaining_params.push(*key);
            }
        }
    }

    (current_value, remaining_params, parent_key)
}