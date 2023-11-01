use super::*;
use std::fs::{read_to_string};
use std::io::Error;

#[cfg(test)]
mod operation_with_temp_file_tests {
    use super::*;
    use std::fs::write;
    use tempfile::NamedTempFile;

    struct TempYamlFile {
        file: NamedTempFile,
    }

    impl TempYamlFile {
        fn path(&self) -> &std::path::Path {
            self.file.path()
        }
    }

    impl Drop for TempYamlFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path());
        }
    }
    
    fn create_temp_yaml_file() -> Result<TempYamlFile, Error> {
        let temp_file = NamedTempFile::new()?;
        write(temp_file.path(), 
              "my-command1:\n  sub-command1: \"echo I am subcommand1\"\n  sub-command2: \"echo I am subcommand2\"")?;
        Ok(TempYamlFile { file: temp_file })
    }
    
    #[test]
    fn test_temp_yaml_file_creation() -> Result<(), Error> {
        let temp_file = create_temp_yaml_file()?;
        assert!(temp_file.path().exists());

        let content = read_to_string(&temp_file.path())?;
        let expected_content = "my-command1:\n  sub-command1: \"echo I am subcommand1\"\n  sub-command2: \"echo I am subcommand2\"";
        assert_eq!(content, expected_content);
        
        Ok(())
    }

    #[test]
    fn test_command_add() -> Result<(), Error> {
        let temp_file = create_temp_yaml_file()?;
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["new-command", "-a", "echo hello new world!"]);
        assert_eq!(result.status, OperationStatus::SpecialOperation);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["new-command"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo hello new world!");
        Ok(())
    }

    #[test]
    fn test_command_add2() -> Result<(), Error> {
        let temp_file = create_temp_yaml_file()?;
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["new-command","-a", "echo hello new world1!"]);
        assert_eq!(result.status, OperationStatus::SpecialOperation);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["new-command", "new-subcommand","-a", "echo hello new world2!"]);
        assert_eq!(result.status, OperationStatus::SpecialOperation);

        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["new-command"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo hello new world1!");

        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["new-command", "new-subcommand"]);
        assert_eq!(result.status, OperationStatus::Command);
        assert_eq!(result.result, "echo hello new world2!");
        Ok(())
    }

    #[test]
    fn test_command_remove_leaf() -> Result<(), Error> {
        let temp_file = create_temp_yaml_file()?;
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["my-command1", "sub-command2", "-r"]);
        assert_eq!(result.status, OperationStatus::SpecialOperation);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["my-command1", "sub-command2"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
        Ok(())
    }

    #[test]
    fn test_command_remove_branch() -> Result<(), Error> {
        let temp_file = create_temp_yaml_file()?;
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["my-command1", "-r"]);
        assert_eq!(result.status, OperationStatus::SpecialOperation);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["my-command1", "sub-command2"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["my-command1"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
        Ok(())
    }

    #[test]
    fn test_command_remove_root() -> Result<(), Error> {
        let temp_file = create_temp_yaml_file()?;
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["-r"]);
        assert_eq!(result.status, OperationStatus::SpecialOperation);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["my-command1", "sub-command2"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &["my-command1"]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
        let result = exec_operation(Mode::Execution, temp_file.path().to_str().unwrap(), &[]);
        assert_eq!(result.status, OperationStatus::NoExecCommands);
        Ok(())
    }
}
