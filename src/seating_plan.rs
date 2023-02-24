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
use crate::file_handler::CoreFileHandle;

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
        let file = match File::open(&file_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Could not open file: {} for {}", e, file_path))
        };
        let seating_plan: SeatingPlan = match serde_yaml::from_reader(file) {
            Ok(s) => s,
            Err(e) => return Err(format!("Could not parse file: {} for {}", e, file_path))
        };
        Ok(seating_plan)
    }

    /// Creates a venue directory if the venue is not already present. 
    /// 
    /// # Arguments
    /// * `file_handler` - A ```CoreFileHandle``` trait object that handles the creation of the venue directory
    /// 
    /// # Returns
    /// * `Result<(), std::io::Error>` - An error if the directory could not be created
    pub fn create_venue(&self, file_handler: &dyn CoreFileHandle) -> Result<(), std::io::Error> {
        println!("Creating venue directory");
        let venue_path = Path::new(&self.venue);
        file_handler.create_directory_if_not_exists(venue_path)
    }
}


// below are tests for the seating_plan.rs file
#[cfg(test)]
mod tests {

    use super::*;
    use crate::file_handler::MockCoreFileHandle;
    use mockall::predicate::eq;

    #[test]
    fn test_from_file() {
        let seating_plan = SeatingPlan::from_file("tests/live_test.yml".to_string()).unwrap();

        assert_eq!(
            seating_plan.attendees,
            vec![
                Dependency {
                    name: "institution".to_string(),
                    url: "https://github.com/yellow-bird-consult/institution.git".to_string(),
                    branch: "infrastructure".to_string(),
                },
            ]
        );

        assert_eq!(
            seating_plan.venue,
            "./sandbox/services/".to_string()
        );
    }

    #[test]
    fn test_create_venue() {
        let seating_plan = SeatingPlan::from_file("tests/live_test.yml".to_string()).unwrap();

        let mut mock_handle = MockCoreFileHandle::new();
        let venue_path = Path::new("./sandbox/services/");

        mock_handle.expect_create_directory_if_not_exists()
            .with(eq(venue_path))
            .returning(|_| {
                Ok(())
            });

        let result = seating_plan.create_venue(&mock_handle);
        assert!(result.is_ok());
        mock_handle.checkpoint(); 
    }
}
