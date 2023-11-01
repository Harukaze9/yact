use super::*;


#[cfg(test)]
mod basic_test {
    use super::*;
    #[test]
    fn test_abbreviated_path() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "abbreviated"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo \"this is abbreviated form\"");
    }

    #[test]
    fn test_regular_path() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "regular"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo regular command");
    }

    #[test]
    fn test_totally_wrong_path() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["wrongcommand", "sample-commands", "abbreviated"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
    }

    #[test]
    fn test_partially_wrong_path() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["subcommand", "sample-commands", "wrongpath"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
    }

    #[test]
    fn test_stump_command() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &["stump-command"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo hello stump");
    }

    #[test]
    fn test_no_param() {
        let result = exec_operation(Mode::Execution, "resources/commands.yaml", &[]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
    }

    #[test]
    fn test_root_command() {
        let result = exec_operation(Mode::Execution, "resources/root_command.yaml", &[]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo I am root!");
    }

    #[test]
    fn test_root_plain_text_command() {
        let result = exec_operation(Mode::Execution, "resources/root_plain_text.yaml", &["wakame"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo I am plain text root command with arg wakame");
    }

    mod multiple {
    use super::*;

        #[test]
        fn test_multiple_command_arg() {
            let result_arg0 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-case"]);
            assert_eq!(result_arg0.status, OperationStatus::Command);
            assert_eq!(result_arg0.result, "echo number of args is 0");

            let result_arg1 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-case", "apple"]);
            assert_eq!(result_arg1.status, OperationStatus::Command);
            assert_eq!(result_arg1.result, "echo number of args is 1: apple");

            let result_arg2 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-case", "apple", "orange"]);
            assert_eq!(result_arg2.status, OperationStatus::Command);
            assert_eq!(result_arg2.result, "echo number of args is 1: apple");

            let result_arg3 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-case", "apple", "orange", "lemon"]);
            assert_eq!(result_arg3.status, OperationStatus::Command);
            assert_eq!(result_arg3.result, "echo number of args is 3: apple, orange, lemon");
        }
        
        #[test]
        fn test_multiple_abbreviated_command() {
            let result_arg0 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-abbreviated-case"]);
            assert_eq!(result_arg0.status, OperationStatus::Command);
            assert_eq!(result_arg0.result, "echo number of args is 0");

            let result_arg1 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-abbreviated-case", "apple"]);
            assert_eq!(result_arg1.status, OperationStatus::Command);
            assert_eq!(result_arg1.result, "echo number of args is 1: apple");

            let result_arg2 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-abbreviated-case", "apple", "orange"]);
            assert_eq!(result_arg2.status, OperationStatus::Command);
            assert_eq!(result_arg2.result, "echo number of args is 1: apple");

            let result_arg3 = exec_operation(Mode::Execution, "resources/commands.yaml", &["multiple-abbreviated-case", "apple", "orange", "lemon"]);
            assert_eq!(result_arg3.status, OperationStatus::Command);
            assert_eq!(result_arg3.result, "echo number of args is 3: apple, orange, lemon");
        }
    }
}


