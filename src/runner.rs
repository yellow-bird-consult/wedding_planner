//! The Runner handles all the processes of the dependencies. 
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::path::Path;

use crate::seating_plan::SeatingPlan;


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
        self.seating_plan.create_venue();
    }

    /// Gets the docker-compose command for the dependencies in the seating plan.
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

    /// Runs a command on the docker-compose files for all the dependencies in the seating plan.
    /// 
    /// # Arguments
    /// * `command` - The command to run on the docker files
    /// * `error_message` - The error message to display if the command fails
    /// * `command_string` - The list of docker-compose files to run the command on
    pub fn run_docker_command(&self, command: &str, error_message: &str, mut command_string: String) {
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

    /// Installs all of the dependencies in the seating plan. 
    pub fn install_dependencies(&self) {
        let venue = &self.seating_plan.venue;

        for dependency in &self.seating_plan.attendees {

            if Path::new(&venue).join(&dependency.name).is_dir() == true {
                std::fs::remove_dir_all(Path::new(&venue).join(&dependency.name)).unwrap();
            };
            // download and checkout the dependency
            dependency.clone_github_repo(&venue);
            dependency.checkout_branch(&venue);
            let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();

            // configure the build files for the dependency
            match wedding_invite.build_files {
                Some(_) => {
                    let locked_build = match wedding_invite.build_lock {
                        Some(unpacked_result) => unpacked_result,
                        None => false
                    };
                    if locked_build == false {
                        wedding_invite.prepare_build_file(&venue, &dependency.name);
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
                        wedding_invite.prepare_init_build_file(&venue, &dependency.name);
                    }
                },
                None => continue
            }
        }
    }

    /// Tears down the dependencies that are running.
    pub fn teardown_dependencies(&self) {
        let command_string = self.get_compose_file_command(false);
        self.run_docker_command(" down", "failed to tear down", command_string);
    }

    /// Tears down the remote dependencies that are running.
    pub fn teardown_remote_dependencies(&self) {
        let command_string = self.get_compose_file_command(true);
        self.run_docker_command(" down", "failed to tear down", command_string);
    }

    /// Builds the dependencies that are needed to run. 
    pub fn build_dependencies(&self) {
        let command_string = self.get_compose_file_command(false);
        self.run_docker_command(" build --no-cache", "failed to build", command_string);
    }

    /// Runs the dependencies defined.
    pub fn run_dependencies(&self) {
        let command_string = self.get_compose_file_command(false);
        self.run_docker_command(" up", "failed to run", command_string);
    }

    /// Runs the remote dependencies defined.
    pub fn run_remote_dependencies(&self) {
        let command_string = self.get_compose_file_command(true);
        self.run_docker_command(" up", "failed to run", command_string);
    }

}
