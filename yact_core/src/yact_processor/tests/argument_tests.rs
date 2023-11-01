use super::*;

#[cfg(test)]
mod argument_tests {
    use super::*;
    #[test]
    fn test_abbreviated_path_with_args() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "abbreviated-with-args", "arg1", "arg2"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo \"this is abbreviated form (args are arg1 and arg2)\"");
    }

    #[test]
    fn test_command_path_with_args() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "with-args", "orange", "apple!"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo arg1 is orange, arg2 is apple!");
    }

    #[test]
    fn test_command_path_with_args_dup() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "with-args-dup", "orange", "apple!"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo orange apple! orange apple!");
    }

    #[test]
    fn test_duplicated_case1() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "test-dup-matches", "orange", "apple"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo your arg is orange and apple");
    }

    #[test]
    fn test_duplicated_case2() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "test-dup-matches", "wakame", "yummy"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo wakame is yummy");
    }

    #[test]
    fn test_duplicated_case3() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "test-dup-matches", "whale", "huge", "cute"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo whale is huge and cute");
    }

    #[test]
    fn test_args_incomplete() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "with-args", "arg1"]);
        assert_eq!(result.status, OperationStatus::IncompleteArgs);
    }

    #[test]
    fn test_no_exec_commands() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "command-with-three-subcommands"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
    }
}