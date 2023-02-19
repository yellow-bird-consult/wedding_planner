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

    /// Installs all of the dependencies in the seating plan. 
    pub fn install_dependencies(&self) {
        let venue = &self.seating_plan.venue;

        for dependency in &self.seating_plan.attendees {

            if Path::new(&venue).join(&dependency.name).is_dir() == true {
                std::fs::remove_dir_all(Path::new(&venue).join(&dependency.name)).unwrap();
            };
            dependency.clone_github_repo(&venue);
            dependency.checkout_branch(&venue);
            let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();
            wedding_invite.prepare_build_file(&venue, &dependency.name);

            match wedding_invite.init_build {
                Some(_) => wedding_invite.prepare_init_build_file(&venue, &dependency.name),
                None => continue
            }
        }
    }

    /// Builds the dependencies that are needed to run. 
    pub fn build_dependencies(&self) {
        let venue = &self.seating_plan.venue;
        let mut command_string = "docker-compose ".to_owned();

        for dependency in &self.seating_plan.attendees {
            let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();
            let files = &wedding_invite.get_docker_compose_files(&venue, &dependency.name);
            command_string.push_str(files);
        }

        command_string.push_str("build --no-cache");

        let mut command = Command::new("bash").arg("-c")
                                                                     .arg(command_string)
                                                                     .stdout(Stdio::piped())
                                                                     .stderr(Stdio::piped()).spawn()
                                                                     .expect("failed to build");
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

    /// Runs the dependencies defined.
    pub fn run_dependencies(&self) {
        let venue = &self.seating_plan.venue;
        let mut command_string = "docker-compose ".to_owned();

        for dependency in &self.seating_plan.attendees {
            let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();
            let files = &wedding_invite.get_docker_compose_files(&venue, &dependency.name);
            command_string.push_str(files);
        }

        command_string.push_str("up");

        let mut command = Command::new("bash").arg("-c")
                                                                     .arg(command_string)
                                                                     .stdout(Stdio::piped())
                                                                     .stderr(Stdio::piped()).spawn()
                                                                     .expect("failed to build");
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
