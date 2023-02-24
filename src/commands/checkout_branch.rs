//! This command checks out a branch in a git repository.
use super::command_runner::CoreRunner;
use std::path::Path;


/// A command to checkout a branch in a repository.
/// 
/// # Fields
/// * `branch_name` - The name of the branch to checkout
/// * `path_to_repo` - The path to the repository to checkout the branch in
/// * `repo_name` - The name of the repository to checkout the branch in
pub struct CheckoutBranchCommand {
    pub branch_name: String,
    pub path_to_repo: String,
    pub repo_name: String
}

impl CheckoutBranchCommand {

    /// Creates a new CheckoutBranchCommand struct.
    /// 
    /// # Arguments
    /// * `branch_name` - The name of the branch to checkout
    /// * `path_to_repo` - The path to the repository to checkout the branch in
    /// * `repo_name` - The name of the repository to checkout the branch in
    /// 
    /// # Returns
    /// A new CheckoutBranchCommand struct
    pub fn new(branch_name: String, path_to_repo: String, repo_name: String) -> Self {
        Self {
            branch_name,
            path_to_repo,
            repo_name
        }
    }

    /// Runs the checkout branch command.
    /// 
    /// # Arguments
    /// * `runner` - The command runner to for the command being run
    /// 
    /// # Returns
    /// The output of the command
    pub fn run(&self, runner: &dyn CoreRunner) -> Result<std::process::Output, std::io::Error> {
        let root_path = Path::new(&self.path_to_repo).join(&self.repo_name).to_string_lossy().to_string();
        let checkout_cmd = format!("cd {} && git checkout {}", root_path, self.branch_name);
        runner.run(&checkout_cmd)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use mockall::predicate::eq;
    use std::os::unix::process::ExitStatusExt;
    use super::super::command_runner::MockCoreRunner;
    use std::process::Output;

    #[test]
    fn test_new() {
        let command = CheckoutBranchCommand::new("test_branch".to_string(), "/path/to/repo".to_string(), "test_repo".to_string());
        assert_eq!(command.branch_name, "test_branch");
        assert_eq!(command.path_to_repo, "/path/to/repo");
        assert_eq!(command.repo_name, "test_repo");
    }

    #[test]
    fn test_run() {
        let command = CheckoutBranchCommand::new("test_branch".to_string(), "/path/to/repo".to_string(), "test_repo".to_string());
        let mut mock_runner = MockCoreRunner::new();
        mock_runner.expect_run()
            .with(eq("cd /path/to/repo/test_repo && git checkout test_branch".to_string()))
            .returning(|_| {
                Ok(Output {
                    status: std::process::ExitStatus::from_raw(0),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            });
        let result = command.run(&mock_runner);
        assert!(result.is_ok());
        mock_runner.checkpoint(); // Ensure all expected calls have been made
    }

}