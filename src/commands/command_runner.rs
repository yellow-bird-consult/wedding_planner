//! Defines the implementation of the CoreRunner trait. This trait is used to run commands and docker commands.
use std::process::{Command, Output, Stdio};
use std::io::prelude::*;


/// Defines the interface for running commands and docker commands.
#[mockall::automock]
pub trait CoreRunner {
    /// Runs a command and returns the output.
    /// 
    /// # Arguments
    /// * `command` - The command to run
    /// 
    /// # Returns
    /// * `Result<Output, std::io::Error>` - The output of the command or an error
    fn run(&self, command: &String) -> Result<Output, std::io::Error>;

    /// Runs a docker command and loops until stopped printing outputs of the docker command in realtime.
    /// 
    /// # Arguments
    /// * `command` - The command to run on the docker files 
    /// * `error_message` - The error message to print if the command fails
    /// * `command_string` - The string to append the output of the command to
    fn run_docker_command(&self, command: &str, error_message: &str, command_string: &mut String) -> ();
}

/// Main implementation for the CoreRunner trait. This struct should be passed into functions that need to run commands.
/// 
/// # Example
/// Below is a simple example of how to use the CommandRunner struct in a function. 
/// 
/// ```rust
/// use crate::commands::command_runner::CoreRunner;
/// 
/// fn run_command(command: &String, runner: &dyn CoreRunner) -> Result<Output, std::io::Error> {
///    runner.run(command)
/// }
/// ```
/// Which can be run using the following example:
/// ```rust
/// use crate::commands::command_runner::CommandRunner;
/// 
/// run_command(&"ls".to_string(), &CommandRunner);
/// ```
/// 
/// # Mocking Example
/// Below is an example of how to mock the CommandRunner struct for testing for the previously defined 
/// ```run_command``` function:
/// 
/// ```rust
/// use crate::commands::command_runner::MockCoreRunner;
/// 
/// #[test]
/// fn test_pass_run_command() {
///     let mut mock_runner = MockCoreRunner::new();
///     let expected_command = "ls";
///     mock_runner.expect_run().with(eq(expected_command.to_string())).returning(|_| {
///         Ok(Output {
///             status: std::process::ExitStatus::from_raw(0),
///             stdout: Vec::new(),
///             stderr: Vec::new(),
///         })
///     });
///     let result = run_command("ls".to_string(), &CommandRunner);
///     assert!(result.is_ok());
///     mock_runner.checkpoint(); // Ensure all expected calls have been made
/// }
/// 
/// #[test]
/// fn test_not_eq_run_command() {
///     let mut mock_runner = MockCoreRunner::new();
///     let expected_command = "lss";
///     mock_runner.expect_run().with(ne(expected_command.to_string())).returning(|_| {
///         Err(std::io::Error::new(std::io::ErrorKind::Other, "Error"))
///     });
///     let result = run_command("ls".to_string(), &CommandRunner);
///     assert!(result.is_err());
///     mock_runner.checkpoint(); // Ensure all expected calls have been made
/// }
/// ```
pub struct CommandRunner;

impl CoreRunner for CommandRunner {
    
    /// Runs a command and returns the output.
    /// 
    /// # Arguments
    /// * `command` - The command to run
    /// 
    /// # Returns
    /// * `Result<Output, std::io::Error>` - The output of the command
    fn run(&self, command: &String) -> Result<Output, std::io::Error> {
        Command::new("sh").arg("-c").arg(command).output()
    }

    /// Runs a docker command and loops until stopped printing outputs of the docker command in realtime.
    /// 
    /// # Arguments
    /// * `command` - The command to run on the docker files
    /// * `error_message` - The error message to print if the command fails
    /// * `command_string` - The string to append the output of the command to
    fn run_docker_command(&self, command: &str, error_message: &str, command_string: &mut String) {
        command_string.push_str(command);

        let mut command = Command::new("bash").arg("-c")
                                                                     .arg(command_string)
                                                                     .stdout(Stdio::piped())
                                                                     .stderr(Stdio::piped()).spawn()
                                                                     .expect(error_message);
        let stdout = command.stdout.take().unwrap();
        let stderr = command.stderr.take().unwrap();
        let mut stdout_reader = std::io::BufReader::new(stdout).lines();
        let mut stderr_reader = std::io::BufReader::new(stderr).lines();

        loop {
            let mut output = String::new();
            if let Some(line) = stdout_reader.next() {
                let unwrapped_line = line.unwrap();
                println!("{}", &unwrapped_line);
                output.push_str(&unwrapped_line);
            }
            if let Some(line) = stderr_reader.next() {
                let unwrapped_line = line.unwrap();
                println!("{}", &unwrapped_line);
                output.push_str(&unwrapped_line);
            }
    
            if output.is_empty() {
                break;
            } else {
                println!("{}", output);
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use mockall::predicate::{eq, ne};

    fn run_command(command: &String, runner: &dyn CoreRunner) -> Result<Output, std::io::Error> {
        runner.run(command)
    }

    #[test]
    fn test_pass_run_command() {
        let mut mock_runner = MockCoreRunner::new();
        let expected_command = "ls";
        mock_runner.expect_run().with(eq(expected_command.to_string())).returning(|_| {
            Ok(Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        });
        let result = run_command(&"ls".to_string(), &mock_runner);
        assert!(result.is_ok());
        mock_runner.checkpoint(); // Ensure all expected calls have been made
    }

    #[test]
    fn test_not_eq_run_command() {
        let mut mock_runner = MockCoreRunner::new();
        let expected_command = "lss";
        mock_runner.expect_run().with(ne(expected_command.to_string())).returning(|_| {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Error"))
        });
        let result = run_command(&"ls".to_string(), &mock_runner);
        assert!(result.is_err());
        mock_runner.checkpoint(); // Ensure all expected calls have been made
    }
}