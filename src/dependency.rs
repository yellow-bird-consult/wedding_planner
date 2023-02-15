use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::path::Path;
use std::fs::File;
use crate::wedding_invite::WeddingInvite;


/// This struct holds the data for a dependency.
///
/// # Fields
/// * `name` - The name of the dependency
/// * `url` - The URL of the dependency Github repository for cloning
/// * `branch` - The branch of the dependency Github repository to clone
/// * `run_config_file` - The location of the docker-compose file to run the dependency
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    name: String,
    url: String,
    branch: String,
    run_config_file: String,
}

impl Dependency {

    /// Clones the dependency repository into the venue directory.
    ///
    /// # Arguments
    /// * `venue_path` - The path to the venue directory
    pub fn clone_github_repo(self, venue_path: String) {
        let repo_path = format!("{}/{}", venue_path, self.name);
        let repo_path = Path::new(&repo_path);

        if repo_path.exists() {
            println!("{} already exists, skipping", self.name);
        }
        else {
            println!("Cloning {} into {}", self.url, venue_path);
            let clone_cmd = format!("git clone {} {}", self.url, repo_path.display());
            let _ = std::process::Command::new("bash")
                .arg("-c")
                .arg(clone_cmd)
                .output()
                .expect("Failed to clone repo");
        }
    }

    /// Gets the WeddingInvite struct from the dependency repository by loading
    /// the ```wedding_invite.yml```file.
    ///
    /// # Arguments
    /// * `repo_path` - The path to the dependency repository
    ///
    /// # Returns
    /// * `Result<WeddingInvite, String>` - A ```WeddingInvite``` struct or an error message
    fn get_wedding_invite(self, repo_path: String) -> Result<WeddingInvite, String> {
        let invite_path = Path::new(&repo_path).join(&self.name)
                                                           .join("wedding_invite.yml");
        let file = match File::open(invite_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Could not open file: {}", e))
        };
        let invite_data: WeddingInvite = match serde_yaml::from_reader(file) {
            Ok(ld) => ld,
            Err(e) => return Err(format!("Could not read values: {}", e))
        };
        Ok(invite_data)
    }
}
