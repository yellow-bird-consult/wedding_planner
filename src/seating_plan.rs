//! A seating plan is a ```yml``` file that defines the dependencies that the program needs to run all of the dependencies for a local run. 
//! ## Example Seating Plan File
//! Below is an example yml file for the seating plan:
//! ```yaml
//!attendees:
//!  - name: John Doe
//!    url: http://example.com/john-doe
//!    branch: development
//!    local_run_config_file: ../sandbox/local_service_configs/jane-doe.yml
//!    remote_run_config_file: ../sandbox/remote_service_configs/jane-doe.yml
//!  - name: Jane Doe
//!    url: http://example.com/jane-doe
//!    branch: development
//!    local_run_config_file: ../sandbox/local_service_configs/jane-doe.yml
//!    remote_run_config_file: ../sandbox/remote_service_configs/jane-doe.yml
//!
//!venue: ../sandbox/services/
//! ```
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;
use std::path::Path;

use crate::dependency::Dependency;


/// This struct holds the data for all dependencies.
///
/// # Fields
/// * `attendees` - A vector of ```Dependency``` structs
/// * `venue` - The directory where all docker-compose files for local services will be run
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SeatingPlan {
    pub attendees: Vec<Dependency>,
    pub venue: String,
}


impl SeatingPlan {

    /// Creates a new SeatingPlan struct from a YAML file.
    ///
    /// # Arguments
    /// * `file_path` - The path to the YAML file
    ///
    /// # Returns
    /// * `Result<SeatingPlan, String>` - A ```SeatingPlan``` struct or an error message
    pub fn from_file(file_path: String) -> Result<SeatingPlan, String> {
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Could not open file: {}", e))
        };
        let seating_plan: SeatingPlan = match serde_yaml::from_reader(file) {
            Ok(s) => s,
            Err(e) => return Err(format!("Could not parse file: {}", e))
        };
        Ok(seating_plan)
    }

    /// Creates a venue directory if the venue is not already present. 
    pub fn create_venue(&self) {
        println!("Creating venue directory");
        let venue_path = Path::new(&self.venue);
        if venue_path.exists() {
            println!("{} already exists, skipping", self.venue);
        }
        else {
            let create_cmd = format!("mkdir {}", venue_path.display());
            let _ = std::process::Command::new("bash")
                .arg("-c")
                .arg(create_cmd)
                .output()
                .expect("Failed to create venue directory");
        }
    }
}


// below are tests for the seating_plan.rs file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file() {
        let seating_plan = SeatingPlan::from_file("tests/live_test.yml".to_string()).unwrap();
        println!("{:?}", seating_plan);
        let venue = seating_plan.venue;
        let dependency = &seating_plan.attendees[0];
        dependency.clone_github_repo(&venue);
        dependency.checkout_branch(&venue);
        let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();

        println!("{:?}", wedding_invite);
        wedding_invite.prepare_build_file(&venue, &dependency.name);
        wedding_invite.prepare_init_build_file(&venue, &dependency.name);
        println!("{:?}", wedding_invite.get_docker_compose_files(&venue, &dependency.name));
    }
}
