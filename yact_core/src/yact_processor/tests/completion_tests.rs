use super::*;

#[cfg(test)]
mod completion_test {
    use super::*;
    #[test]
    fn test_command_completion_for_arg1() {
    let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["subcommand", "command-with-input-completion"]);
        assert_eq!(result.status, OperationStatus::CompletionCommand);
        assert_eq!(result.result, "echo apple orange lemon");
    }

    #[test]
    fn test_command_completion_for_arg2() {
    let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["subcommand", "command-with-input-completion", "apple"]);
        assert_eq!(result.status, OperationStatus::CompletionCommand);
        assert_eq!(result.result, "echo banana grape");
    }

    #[test]
    fn test_command_completion_default1() {
    let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["subcommand", "sample-commands", "with-args"]);
        assert_eq!(result.status, OperationStatus::CompletionDefault);
    }

    #[test]
    fn test_command_completion_default2() {
    let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["subcommand", "sample-commands", "with-args", "arg1"]);
        assert_eq!(result.status, OperationStatus::CompletionDefault);
    }

    #[test]
    fn test_command_completion_default3() {
    let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["subcommand", "sample-commands", "with-args", "arg1", "arg2"]);
        assert_eq!(result.status, OperationStatus::CompletionText);
        assert_eq!(result.result, "");
    }

    #[test]
    fn test_command_completion_subcommands() {
    let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["subcommand", "command-with-three-subcommands"]);
        assert_eq!(result.status, OperationStatus::CompletionText);
        assert_eq!(result.result, "subcommand1 subcommand2 subcommand3");
    }

    #[test]
    fn test_command_completion_for_arg3() {
    let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["subcommand", "command-with-input-completion", "apple", "banana"]);
        assert_eq!(result.status, OperationStatus::CompletionDefault);
        assert_eq!(result.result, "");
    }

    #[test]
    fn test_command_completion_no_params() {
    let result = exec_operation(Mode::Completion, "resources/no_param_completion_test.yaml", &[]);
        assert_eq!(result.status, OperationStatus::CompletionText);
        assert_eq!(result.result, "hello1 orange apple");
    }

    #[test]
    fn test_root_command() {
        let result = exec_operation(Mode::Completion, "resources/root_command.yaml", &[]);
        assert_eq!(result.status, OperationStatus::CompletionText);
        assert_eq!(result.result, "");
    }

    #[test]
    fn test_root_plain_text_command() {
        let result = exec_operation(Mode::Completion, "resources/root_plain_text.yaml", &[]);
        assert_eq!(result.status, OperationStatus::CompletionDefault);
        assert_eq!(result.result, "");
    }

    #[test]
    fn test_root_plain_text_command2() {
        let result = exec_operation(Mode::Completion, "resources/root_plain_text.yaml", &["wakame"]);
        assert_eq!(result.status, OperationStatus::CompletionText);
        assert_eq!(result.result, "");
    }

    mod complete_options {
        use super::*;
        #[test]
        fn test_special_operation_root() {
            let result = exec_operation(Mode::Completion, "resources/commands.yaml", &[]);
            assert_eq!(result.status, OperationStatus::CompletionText);
            assert!(result.message.contains("-p")); // only if path exists
            assert!(result.message.contains("-a")); // always available
            assert!(result.message.contains("-r")); // only if path exists
        }

        #[test]
        fn test_special_operation_executable_command() {
            let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["stump-command"]);
            assert_eq!(result.status, OperationStatus::CompletionText);
            assert!(result.message.contains("-p")); // only if path exists
            assert!(result.message.contains("-a")); // always available
            assert!(result.message.contains("-r")); // only if path exists
        }

        #[test]
        fn test_special_operation_at_wrong_path() {
            let result = exec_operation(Mode::Completion, "resources/commands.yaml", &["stump-commandXXX"]);
            assert_eq!(result.status, OperationStatus::CompletionText);
            assert!(!result.message.contains("-p")); // only if path exists
            assert!(result.message.contains("-a")); // always available
            assert!(!result.message.contains("-r")); // only if path exists
        }

    }
}

