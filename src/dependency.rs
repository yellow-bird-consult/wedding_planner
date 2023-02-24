//! A dependency is the data around a github repo that is going to be pulled as a dependency.
//! For the dependency we can perform the following tasks:
//! - clone the Github repository
//! - checkout a branch for the Github repository
//! - Gets the wedding invite data from the Github repository
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::wedding_invite::WeddingInvite;
use crate::commands::{
    command_runner::CoreRunner,
    checkout_branch::CheckoutBranchCommand,
    clone_repo::CloneRepoCommand
};


/// This struct holds the data for a dependency.
///
/// # Fields
/// * `name` - The name of the dependency
/// * `url` - The URL of the dependency Github repository for cloning
/// * `branch` - The branch of the dependency Github repository to clone
/// * `run_config_file` - The location of the docker-compose file to run the dependency
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    pub name: String,
    pub url: String,
    pub branch: String,
    // run_config_file: String,
}

impl Dependency {

    /// Clones the dependency repository into the venue directory.
    ///
    /// # Arguments
    /// * `venue_path` - The path to the venue directory
    /// 
    /// # Returns
    /// The result of the clone command
    pub fn clone_github_repo(&self, venue_path: &String, runner: &dyn CoreRunner) -> Result<(), std::io::Error> {
        let repo_path = Path::new(&venue_path).join(&self.name);

        if repo_path.exists() {
            println!("{} already exists, skipping", self.name);
            return Ok(());
        }
        else {
            let clone_command = CloneRepoCommand::new(
                self.url.clone(), 
                venue_path.clone()
            );
            match clone_command.run(runner) {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
            }
        }
    }

    /// Gets the WeddingInvite struct from the dependency repository by loading
    /// the ```wedding_invite.yml```file.
    ///
    /// # Arguments
    /// * `venue_path` - The path to the dependency repository
    ///
    /// # Returns
    /// * `Result<WeddingInvite, String>` - A ```WeddingInvite``` struct or an error message
    pub fn get_wedding_invite(&self, venue_path: &String) -> Result<WeddingInvite, String> {
        let invite_path = Path::new(&venue_path).join(&self.name)
                                                           .join("wedding_invite.yml");
        if invite_path.exists() == false {
            return Err(format!("{} does not exist", invite_path.to_str().unwrap()));
        }
        let invite_data = match WeddingInvite::from_file(invite_path.to_str().unwrap().to_string()) {
            Ok(ld) => ld,
            Err(e) => return Err(format!("Could not read values: {}", e))
        };
        Ok(invite_data)
    }

    /// Checks out the branch of the dependency repository.
    /// 
    /// # Arguments
    /// * `venue_path` - The path to the dependency repository
    /// 
    /// # Returns
    /// None
    pub fn checkout_branch(&self, venue_path: &String, runner: &dyn CoreRunner) -> Result<std::process::Output, std::io::Error> {
        CheckoutBranchCommand::new(
            self.branch.clone(), 
            venue_path.clone(), 
            self.name.clone()).run(runner)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::os::unix::process::ExitStatusExt;
    use crate::commands::command_runner::MockCoreRunner;
    use crate::wedding_invite::InitBuild;
    use std::process::Output;
    use mockall::predicate::eq;

    static TEST_NAME: &str = "test_repo";
    static REPO_URL: &str = "https://github.com/yellow-bird-consult/wedding_planner";
    static BRANCH: &str = "master";

    #[test]
    fn test_get_wedding_invite() {
        let dependency = Dependency {
            name: TEST_NAME.to_string(),
            url: REPO_URL.to_string(),
            branch: BRANCH.to_string()
        };
        let venue_path = "./tests/".to_string();
        let wedding_invite = dependency.get_wedding_invite(&venue_path).unwrap();

        let mut normal_builds = HashMap::new();
        normal_builds.insert("x86_64".to_string(), "build/Dockerfile.x86_64".to_string());
        normal_builds.insert("aarch64".to_string(), "build/Dockerfile.aarch64".to_string());

        let mut init_builds = HashMap::new();
        init_builds.insert("x86_64".to_string(), "database/build/Dockerfile.init".to_string());
        init_builds.insert("aarch64".to_string(), "database/build/Dockerfile.init.arch".to_string());

        assert_eq!(wedding_invite.build_files, Some(normal_builds));

        assert_eq!(wedding_invite.build_root, ".");
        assert_eq!(wedding_invite.init_build, Some(InitBuild {
            build_files: init_builds,
            build_root: "database".to_string(),
            build_lock: None
        }));

        // compare the runner_files to the expected runner_files
        let expected_runner_files = vec![
            "runner_files/base.yml".to_string(),
            "runner_files/database.yml".to_string(),
        ];
        assert_eq!(wedding_invite.runner_files, expected_runner_files);

        let venue_path = "/should/not/exist/".to_string();
        assert_eq!(dependency.get_wedding_invite(&venue_path), Err("/should/not/exist/test_repo/wedding_invite.yml does not exist".to_string()))

    }

    #[test]
    fn test_clone_github_repo() {
        let dependency = Dependency {
            name: TEST_NAME.to_string(),
            url: REPO_URL.to_string(),
            branch: BRANCH.to_string()
        };
        let venue_path = "some/path/to/repo".to_string();
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
        let result = dependency.clone_github_repo(&venue_path, &mock_runner);
        assert!(result.is_ok());
        mock_runner.checkpoint(); 
    }

    #[test]
    fn test_checkout_branch() {
        let dependency = Dependency {
            name: TEST_NAME.to_string(),
            url: REPO_URL.to_string(),
            branch: BRANCH.to_string()
        };
        let venue_path = "some/path/to/repo".to_string();
        let mut mock_runner = MockCoreRunner::new();

        mock_runner.expect_run()
            .with(eq("cd some/path/to/repo/test_repo && git checkout master".to_string()))
            .returning(|_| {
                Ok(Output {
                    status: std::process::ExitStatus::from_raw(0),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                })
            });
        let result = dependency.checkout_branch(&venue_path, &mock_runner);
        assert!(result.is_ok());
        mock_runner.checkpoint(); 
    }
}