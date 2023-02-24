//! This command clones a git repository.
use super::command_runner::CoreRunner;


/// A command to clone a git repository.
/// 
/// # Fields
/// * `repo_url` - The URL of the repository to clone
/// * `path_to_repo` - The local path to where the repository should be cloned to
pub struct CloneRepoCommand {
    pub repo_url: String,
    pub path_to_repo: String
}


impl CloneRepoCommand {

    /// Creates a new CloneRepoCommand struct.
    /// 
    /// # Arguments
    /// * `repo_url` - The URL of the repository to clone
    /// * `path_to_repo` - The path to the repository to clone
    /// 
    /// # Returns
    /// A new CloneRepoCommand struct
    pub fn new(repo_url: String, path_to_repo: String) -> Self {
        Self {
            repo_url,
            path_to_repo
        }
    }

    /// Runs the clone repo command.
    /// 
    /// # Arguments
    /// * `runner` - The command runner to for the command being run
    /// 
    /// # Returns
    /// The output of the command
    pub fn run(&self, runner: &dyn CoreRunner) -> Result<std::process::Output, std::io::Error> {
        let clone_cmd = format!("cd {} && git clone {}", self.path_to_repo, self.repo_url);
        runner.run(&clone_cmd)
    }
}
    

#[cfg(test)]
mod tests {

    use super::*;
    use mockall::predicate::eq;
    use std::os::unix::process::ExitStatusExt;
    use super::super::command_runner::MockCoreRunner;
    use std::process::Output;

    static REPO_URL: &str = "https://github.com/yellow-bird-consult/wedding_planner";
    static PATH_TO_REPO: &str = "some/path/to/repo";

    #[test]
    fn test_new() {
        let command = CloneRepoCommand::new(
            REPO_URL.to_string(), 
            PATH_TO_REPO.to_string()
        );
        assert_eq!(command.repo_url, REPO_URL);
        assert_eq!(command.path_to_repo, PATH_TO_REPO);
    }

    #[test]
    fn test_run() {
        let command = CloneRepoCommand::new(
            REPO_URL.to_string(), 
            PATH_TO_REPO.to_string()
        );
        let mut mock_runner = MockCoreRunner::new();

        mock_runner.expect_run()
            .with(eq("cd some/path/to/repo && git clone https://github.com/yellow-bird-consult/wedding_planner".to_string()))
            .returning(|_| {
                Ok(Output {
                    status: std::process::ExitStatus::from_raw(0),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            });
        let result = command.run(&mock_runner);
        assert!(result.is_ok());
        mock_runner.checkpoint(); 
    }
}