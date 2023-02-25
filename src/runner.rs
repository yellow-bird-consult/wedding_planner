//! The Runner handles all the processes of the dependencies. 
use std::{env, path::Path};

use crate::seating_plan::SeatingPlan;
use crate::commands::command_runner::{
    CoreRunner,
    CommandRunner
};
use crate::file_handler::FileHandle;


/// Runs the processes for seating plan and thus runs the processes around running dependencies.
/// 
/// # Fields 
/// * `seating_plan` - The seating plan that defines the dependencies to run
pub struct Runner {
    pub seating_plan: SeatingPlan
}


impl Runner {

    /// The constructor for the Runner struct.
    /// 
    /// # Arguments
    /// * `path` - The path to the seating plan file
    /// 
    /// # Returns
    /// * `Runner` - A Runner struct wrapped in a result
    pub fn new(path: String) -> Result<Runner, String> {
        match SeatingPlan::from_file(path){
            Ok(seating_plan) => Ok(Runner{seating_plan}),
            Err(error) => Err(error)
        }
    }

    /// Creates the venue directory.
    pub fn create_venue(&self) {
        match self.seating_plan.create_venue(&FileHandle{}){
            Ok(_) => {
                println!("Created venue directory");
            },
            Err(error) => println!("Failed to create venue: {}", error)
        };
    }

    /// Gets the docker-compose command for the dependencies in the seating plan.
    /// 
    /// # Arguments
    /// * `remote` - If true the remote docker-compose files meaning the docker-compose files that rely on images from Dockerhub
    /// 
    /// # Returns
    /// * `String` - The docker-compose command
    /// 
    /// # Example
    /// ```
    /// docker-compose -f venue/dependency1/docker-compose.yml -f venue/dependency2/docker-compose.yml
    /// ```
    pub fn get_compose_file_command(&self, remote: bool) -> String {
        let venue = &self.seating_plan.venue;
        let mut command_string = "docker-compose ".to_owned();

        for dependency in &self.seating_plan.attendees {
            let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();

            let files = match remote {
                true => wedding_invite.get_remote_compose_files(&venue, &dependency.name),
                false => wedding_invite.get_docker_compose_files(&venue, &dependency.name)
            };
            command_string.push_str(&files);
        }
        return command_string;
    }

    /// Installs all of the dependencies in the seating plan. 
    pub fn install_dependencies(&self) {
        let cwd = env::current_dir().unwrap().to_str().unwrap().to_owned();
        let venue = &self.seating_plan.venue;
        let full_venue_path = Path::new(&cwd).join(&venue).to_string_lossy().to_string();

        let command_runner = CommandRunner {};
        let file_handle = FileHandle {};

        for dependency in &self.seating_plan.attendees {

            if Path::new(&venue).join(&dependency.name).is_dir() == true {
                std::fs::remove_dir_all(Path::new(&venue).join(&dependency.name)).unwrap();
            };
            // download and checkout the dependency
            match dependency.clone_github_repo(&full_venue_path, &command_runner) {
                Ok(_) => {
                    println!("Cloned repo for {}/{}", &full_venue_path, dependency.name);
                },
                Err(error) => {
                    println!("Failed to clone repo for {}: {}", dependency.name, error);
                    continue
                }
            }
            match dependency.checkout_branch(&full_venue_path, &command_runner){
                Ok(_) => {
                    println!("Checked out branch for {}/{} as branch {}", &full_venue_path, dependency.name, dependency.branch);
                },
                Err(error) => {
                    println!("Failed to checkout branch for {} as branch {}: {}", dependency.name, dependency.branch, error);
                    continue
                }
            };
            let wedding_invite = dependency.get_wedding_invite(&full_venue_path).unwrap();

            // configure the build files for the dependency
            match wedding_invite.build_files {
                Some(_) => {
                    let locked_build = match wedding_invite.build_lock {
                        Some(unpacked_result) => unpacked_result,
                        None => false
                    };
                    if locked_build == false {
                        let _ = wedding_invite.prepare_build_file(&full_venue_path, &dependency.name, &file_handle);
                    }
                },
                None => continue
            }
            // configure the build files for the dependency's init build
            match &wedding_invite.init_build {
                Some(unpacked_init_build) => {
                    let locked_build = match unpacked_init_build.build_lock {
                        Some(unpacked_result) => unpacked_result,
                        None => false
                    };
                    if locked_build == false {
                        match wedding_invite.prepare_init_build_file(&full_venue_path, &dependency.name, &file_handle) {
                            Ok(_) => {
                                println!("Prepared init build file for {}", dependency.name);
                            },
                            Err(error) => {
                                println!("Failed to prepare init build file for {}: {}", dependency.name, error);
                                continue
                            }
                        };
                    }
                },
                None => continue
            }
        }
    }

    /// Tears down the dependencies that are running.
    /// 
    /// # Process
    /// 1. gets all the runner_files in the wedding invites of the dependencies
    /// 2. runs the docker command to tear down the dependencies
    pub fn teardown_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" down", "failed to tear down", &mut command_string);
    }

    /// Tears down the remote dependencies that are running.
    /// 
    /// # Process
    /// 1. gets all the remote_runner_files in the wedding invites of the dependencies
    /// 2. runs the docker command to tear down the dependencies
    pub fn teardown_remote_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(true);
        command_runner.run_docker_command(" down", "failed to tear down", &mut command_string);
    }

    /// Builds the dependencies that are needed to run.
    /// 
    /// # Process
    /// 1. gets all the runner_files in the wedding invites of the dependencies
    /// 2. runs the docker command to build the dependencies
    pub fn build_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" build", "failed to build", &mut command_string);
    }

    /// Runs the dependencies defined.
    /// 
    /// # Process
    /// 1. gets all the runner_files in the wedding invites of the dependencies
    /// 2. runs the docker command to run the dependencies
    pub fn run_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" up", "failed to run", &mut command_string);
    }

    /// Runs the dependencies defined in the background.
    /// 
    /// # Process
    /// 1. gets all the runner_files in the wedding invites of the dependencies
    /// 2. runs the docker command to run the dependencies in the background
    pub fn run_dependencies_background(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" up -d", "failed to run", &mut command_string);
    }

    /// Runs the remote dependencies defined.
    /// 
    /// # Process
    /// 1. gets all the remote_runner_files in the wedding invites of the dependencies
    /// 2. runs the docker command to run the dependencies
    pub fn run_remote_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(true);
        command_runner.run_docker_command(" up", "failed to run", &mut command_string);
    }

    /// Runs the remote dependencies defined in the background.
    /// 
    /// # Process
    /// 1. gets all the remote_runner_files in the wedding invites of the dependencies
    /// 2. runs the docker command to run the dependencies in the background
    pub fn run_remote_dependencies_background(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(true);
        command_runner.run_docker_command(" up -d", "failed to run", &mut command_string);
    }

}
