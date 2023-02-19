//! A seating plan is a ```yml``` file that defines the dependencies that the program needs to run all of the dependencies for a local run. 
//! TODO -> put in an example yml file for what the file looks like when the program is finished. 
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

    pub fn create_venue(self) {
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
        // &seating_plan.attendees[0].clone_github_repo(seating_plan.venue);
        // let expected_seating_plan = SeatingPlan {
        //     attendees: vec![
        //         Dependency {
        //             name: "wedding_invite".to_string(),
        //             url: "
    }
}