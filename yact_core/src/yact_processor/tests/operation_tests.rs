// use super::*;
// use std::io::Error;

// #[cfg(test)]
// mod operation_tests {
//     use super::*;

// #[test]
// fn test_command_copy() -> Result<(), Error> {
//     let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "abbreviated", "--copy"]);
//     assert_eq!(result.status, OperationStatus::SpecialOperationClipboard);
//     assert_eq!(result.result, "echo \"this is abbreviated form\"");
//     Ok(())
// }

// #[test]
// fn test_command_copy2() -> Result<(), Error> {
//     let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "dummy", "--copy"]);
//     assert_eq!(result.status, OperationStatus::Error);
//     Ok(())
// }
// }