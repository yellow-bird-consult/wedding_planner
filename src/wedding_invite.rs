//! Wedding invites are ```yml``` files that sit in the root of a github repository that is going to be pulled as a dependency. 
//! TODO -> put in an example file in this documentation when the complete program is working.
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::{File, copy};
use std::collections::HashMap;
use std::path::Path;


/// A struct to hold the local data around a build for an init pod.
///
/// # Fields
/// * `build_files` - A map of Dockerfiles relating to CPU information
/// * `build_root` - The root of the build (where the Dockerfile needs to be to run)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InitBuild {
    pub build_files: HashMap<String, String>,
    pub build_root: String
}


/// A struct to hold the local data around a build.
///
/// # Fields
/// * `build_files` - A map of Dockerfiles relating to CPU information
/// * `build_root` - The root of the build (where the Dockerfile needs to be to run)
/// * `package_file` - The location of the docker-compose file to run the build
/// * `init_build` - The location of the data needed for an init pod build
/// * `runner_files` - The location of the docker-compose files to run the build
#[derive(Debug, Serialize, Deserialize)]
pub struct WeddingInvite {
    pub build_files: Option<HashMap<String, String>>,
    pub build_root: String,
    pub init_build: Option<InitBuild>,
    pub runner_files: Vec<String>
}


impl WeddingInvite {

    /// Create a new WeddingInvite struct from a file
    ///
    /// # Arguments
    /// * `path` - The path to the file to read
    ///
    /// # Returns
    /// * `Result<WeddingInvite, String>` - A WeddingInvite struct or an error message
    pub fn from_file(path: String) -> Result<Self, String> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Could not open file: {}", e))
        };
        let invite_data: WeddingInvite = match serde_yaml::from_reader(file) {
            Ok(ld) => ld,
            Err(e) => return Err(format!("Could not read values: {}", e))
        };
        Ok(invite_data)
    }

    /// Copies the correct Dockerfile to the build root.
    ///
    /// # Arguments
    /// * `repo_local_path` - The path to the local repository
    pub fn prepare_build_file(&self, venue_path: &String, name: &String) {
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let cpu_type = super::cpu_data::CpuType::get().to_string();
        let files_map = self.build_files.as_ref().unwrap();
        let build_file_path = match files_map.get(&cpu_type){
            Some(p) => p,
            None => panic!("No build file for CPU type: {}", &cpu_type)
        };
        let build_path = Path::new(&invite_path).join(build_file_path);
        let build_root_path = Path::new(&invite_path).join(&self.build_root)
                                                                    .join("Dockerfile");
        copy(build_path, build_root_path).unwrap();
    }

    /// Copies the correct Dockerfile to the build root.
    ///
    /// # Arguments
    /// * `repo_local_path` - The path to the local repository
    pub fn prepare_init_build_file(&self, venue_path: &String, name: &String) {
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let cpu_type = super::cpu_data::CpuType::get().to_string();
        let build_file_path = match self.init_build.as_ref().unwrap().build_files.get(&cpu_type){
            Some(p) => p,
            None => panic!("No build file for CPU type: {}", &cpu_type)
        };
        let build_path = Path::new(&invite_path).join(build_file_path);
        let build_root_path = Path::new(&invite_path).join(&self.init_build.as_ref().unwrap().build_root)
                                                                    .join("Dockerfile");
        copy(build_path, build_root_path).unwrap();
    }

    /// Gets the docker-compose files command string.
    /// 
    /// # Returns
    /// * `String` - The docker-compose files command string
    pub fn get_docker_compose_files(&self, venue_path: &String, name: &String) -> String {
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let mut files_string = String::new();
        for file in &self.runner_files {
            files_string.push_str(&format!("-f {}/{} ", &invite_path, file));
        }
        files_string
    }
    // /// Builds the docker image for the project.
    // ///
    // /// # Arguments
    // /// * `repo_local_path` - The path to the local repository
    // /// * `image_name` - The name of the image to build
    // pub fn build_docker_image(self, repo_local_path: String, image_name: &String) {
    //     self.prepare_build_file(repo_local_path.clone());
    //
    //     let build_root_path = Path::new(&repo_local_path).join(&self.build_root);
    //
    //     let mut build_command = std::process::Command::new("docker");
    //     build_command.arg("build");
    //     build_command.arg("-t");
    //     build_command.arg(image_name);
    //     build_command.arg(".");
    //     build_command.current_dir(build_root_path);
    //     let output = build_command.output().unwrap();
    //     println!("Output: {}", String::from_utf8(output.stdout).unwrap());
    //     println!("Error: {}", String::from_utf8(output.stderr).unwrap());
    // }
    //
    // /// Builds the docker image for the init pod.
    // ///
    // /// # Arguments
    // /// * `repo_local_path` - The path to the local repository
    // /// * `image_name` - The name of the image to build
    // pub fn build_init_docker_image(self, repo_local_path: String, image_name: &String) {
    //     match &self.init_build {
    //         Some(init_build) => {
    //             self.prepare_init_build_file(repo_local_path.clone());
    //             let build_root_path = Path::new(&repo_local_path).join(&init_build.build_root);
    //
    //             let mut build_command = std::process::Command::new("docker");
    //             build_command.arg("build");
    //             build_command.arg("-t");
    //             build_command.arg(image_name);
    //             build_command.arg(".");
    //             build_command.current_dir(build_root_path);
    //             let output = build_command.output().unwrap();
    //             println!("Output: {}", String::from_utf8(output.stdout).unwrap());
    //             println!("Error: {}", String::from_utf8(output.stderr).unwrap());
    //         },
    //         None => println!("No init build data")
    //     };
    // }
}


#[cfg(test)]
mod local_data_tests {
    
    use super::*;

    #[test]
    fn test_from_file() {
        let mut normal_builds = HashMap::new();
        normal_builds.insert("x86_64".to_string(), "build/Dockerfile.x86_64".to_string());
        normal_builds.insert("aarch64".to_string(), "build/Dockerfile.aarch64".to_string());

        let mut init_builds = HashMap::new();
        init_builds.insert("x86_64".to_string(), "database/build/Dockerfile.init".to_string());
        init_builds.insert("aarch64".to_string(), "database/build/Dockerfile.init.arch".to_string());

        let ld = WeddingInvite::from_file("./tests/wedding_invite.yml".to_string()).unwrap();
        assert_eq!(ld.build_files, Some(normal_builds));

        assert_eq!(ld.build_root, ".");
        assert_eq!(ld.init_build, Some(InitBuild {
            build_files: init_builds,
            build_root: "database".to_string()
        }));
    }

    #[test]
    fn test_from_file_missing() {
        let ld = WeddingInvite::from_file("./tests/wedding_invite_missing.yml".to_string());
        assert!(ld.is_err());
    }
}