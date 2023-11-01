extern crate clap;
extern crate serde;
extern crate serde_yaml;

use clap::{App, Arg, AppSettings};
use log::{info, error};
use base64;

use crate::yact_processor::OperationStatus;
mod yact_processor;

fn main() {
    #[cfg(debug_assertions)]
    {
        std::env::set_var("RUST_LOG", "info");
    }
    #[cfg(not(debug_assertions))]
    {
        std::env::set_var("RUST_LOG", "off");
    }
    env_logger::init();

    let matches = App::new("jact_engine")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Retrieve values from a YAML file using provided keys.")
        .setting(AppSettings::AllowLeadingHyphen)  
        .arg(Arg::with_name("MODE")
            .help("Operation mode: execution or completion")
            .required(true)
            .possible_values(&["execution", "completion"])
            .index(1))
        .arg(Arg::with_name("FILE")
            .help("Path to the YAML file")
            .required(true)
            .index(2))
        .arg(Arg::with_name("PARAMS")
            .help("Params to access value inside the YAML file")
            .multiple(true)
            .required(false)
            .allow_hyphen_values(true)
            .index(3))
        .get_matches();

    let mode_str = matches.value_of("MODE").unwrap();
    let mode = yact_processor::Mode::from_str(mode_str).expect("Invalid mode provided");
    let yaml_path = matches.value_of("FILE").unwrap();
    let params: Vec<&str> = matches.values_of("PARAMS").unwrap_or_default().collect();

    
    let operation_result = yact_processor::exec_operation(mode, yaml_path, &params); 
    if operation_result.status == OperationStatus::Error {
        error!("An error occurred: {:?}", operation_result.result);
        error!("Provided arguments:");
        error!("\tOperation Mode: {}", mode_str);
        error!("\tReading YAML file from: {}", yaml_path);
        error!("\tparameters: {:?}", params);
    }

    info!("\nstatus: {}\nresult: {}\nmessage: {}", operation_result.status.to_string(), &operation_result.result, &operation_result.message);

    // Pass parameters to the shell function using Base64 encoding separated by spaces
    let combined_encoded_string = format!(
        "{}\n{}\n{}",
        base64::encode(operation_result.status.to_string()),
        base64::encode(&operation_result.result),
        base64::encode(&operation_result.message)
    );
    println!("{}", combined_encoded_string);
    
}


impl yact_processor::Mode {
    fn from_str(mode_str: &str) -> Option<yact_processor::Mode> {
        match mode_str {
            "execution" => Some(yact_processor::Mode::Execution),
            "completion" => Some(yact_processor::Mode::Completion),
            _ => None,
        }
    }
}