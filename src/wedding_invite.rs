//! Wedding invites are ```yml``` files that sit in the root of a github repository that is going to be pulled 
//! as a dependency. 
//! ## Example Seating Plan File
//! Below is an example yml file for the seating plan that should be in the root of the Github repository 
//! of the dependency:
//! ```yaml
//! build_root: "."
//! runner_files:
//!   - runner_files/base.yml
//!   - runner_files/database.yml
//! build_files:
//!   x86_64: builds/Dockerfile.x86_64
//!   aarch64: builds/Dockerfile.aarch64
//! init_build:
//!   build_files:
//!     x86_64: builds/Dockerfile.x86_64
//!     aarch64: builds/Dockerfile.aarch64
//!   build_root: database
//! ```
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use crate::file_handler::CoreFileHandle;


/// A struct to hold the local data around a build for an init pod.
///
/// # Fields
/// * `build_files` - A map of Dockerfiles relating to CPU information
/// * `build_root` - The root of the build (where the Dockerfile needs to be to run)
/// * `build_lock` - Whether to lock the build to a specific CPU architecture, if ```true``` the CPU will not be checked and the Dockerfile will not be moved
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InitBuild {
    pub build_files: HashMap<String, String>,
    pub build_root: String,
    pub build_lock: Option<bool>
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TestBuild {
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
/// * `remote_runner_files` - The location of the docker-compose files to run the build from a remote dockerhub repository
/// * `build_lock` - Whether to lock the build to a specific CPU architecture, if ```true``` the CPU will not be checked and the Dockerfile will not be moved
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WeddingInvite {
    pub build_files: Option<HashMap<String, String>>,
    pub build_root: String,
    pub init_build: Option<InitBuild>,
    pub runner_files: Vec<String>,
    pub remote_runner_files: Option<Vec<String>>,
    pub build_lock: Option<bool>,
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
    /// * `venue_path` - The path to the venue directory where all the dependencies are stored
    /// * `name` - The name of the dependency in the venue directory
    /// * `handle` - A FileHandle struct to handle the copying of the build file
    /// 
    /// # Returns
    /// * `io::Result<u64>` - The number of bytes copied
    pub fn prepare_build_file(&self, venue_path: &String, name: &String, handle: &dyn CoreFileHandle) -> std::io::Result<u64> {
        if let Some(lock) = self.build_lock {
            if lock == true {
                return Ok(0)
            }
        }
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let cpu_type = super::cpu_data::CpuType::get().to_string();
        let files_map = self.build_files.as_ref().unwrap();
        let build_file_path = match files_map.get(&cpu_type){
            Some(p) => p,
            None => return Err(std::io::Error::new(std::io::ErrorKind::Other, 
                format!("No build file for CPU type: {}", cpu_type)))
        };
        let build_path = Path::new(&invite_path).join(build_file_path);
        let build_root_path = Path::new(&invite_path).join(&self.build_root)
                                                                    .join("Dockerfile");
        handle.copy(&build_path, &build_root_path)
    }

    /// Deletes the Dockerfile from the build root.
    /// 
    /// # Arguments
    /// * `venue_path` - The path to the venue where all dependencies are stored
    /// * `name` - The name of the repository where we can prepare the init build
    /// * `handle` - A FileHandle struct to handle the removing of the build file
    /// 
    /// # Returns
    /// * `io::Result<()>` - An empty result or an error
    pub fn delete_build_file(&self, venue_path: &String, name: &String, handle: &dyn CoreFileHandle) -> Result<(), std::io::Error> {
        if let Some(lock) = self.build_lock {
            if lock == true {
                return Ok(())
            }
        }
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let build_root_path = Path::new(&invite_path).join(&self.build_root)
                                                                    .join("Dockerfile");
        handle.remove(&build_root_path)
    }

    /// Copies the correct Dockerfile to the build root.
    /// 
    /// # Arguments
    /// * `venue_path` - The path to the venue where all dependencies are stored
    /// * `name` - The name of the repository where we can prepare the init build
    /// * `handle` - A FileHandle struct to handle the copying of the build file
    /// 
    /// # Returns
    /// * `io::Result<u64>` - The number of bytes copied
    pub fn prepare_init_build_file(&self, venue_path: &String, name: &String, handle: &dyn CoreFileHandle) -> std::io::Result<u64> {

        if None == self.init_build {
            return Ok(0)
        }
        if let Some(lock) = self.init_build.as_ref().unwrap().build_lock {
            if lock == true {
                return Ok(0)
            }
        }
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let cpu_type = super::cpu_data::CpuType::get().to_string();

        let build_file_path = match self.init_build.as_ref().unwrap().build_files.get(&cpu_type){
            Some(p) => p,
            None => panic!("No build file for CPU type: {}", &cpu_type)
        };

        let build_path = Path::new(&invite_path).join(build_file_path);
        let build_root_path = Path::new(&invite_path).join(&self.init_build.as_ref().unwrap().build_root)
                                                                    .join("Dockerfile");
        handle.copy(&build_path, &build_root_path)
    }

    /// Deletes the Dockerfile from the init build root.
    /// 
    /// # Arguments
    /// * `venue_path` - The path to the venue where all dependencies are stored
    /// * `name` - The name of the repository where we can prepare the init build
    /// * `handle` - A FileHandle struct to handle the removing of the build file
    pub fn delete_init_build_file(&self, venue_path: &String, name: &String, handle: &dyn CoreFileHandle) -> Result<(), std::io::Error> {
        if None == self.init_build {
            return Ok(())
        }
        if let Some(lock) = self.init_build.as_ref().unwrap().build_lock {
            if lock == true {
                return Ok(())
            }
        }
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let build_root_path = Path::new(&invite_path).join(&self.init_build.as_ref().unwrap().build_root)
                                                                    .join("Dockerfile");
        handle.remove(&build_root_path)
    }

    /// Gets the docker-compose files command string.
    /// 
    /// # Arguments
    /// * `venue_path` - The path to the venue where all dependencies are stored
    /// * `name` - The name of the repository where we can run the images
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

    /// Gets the docker-compose files command string that run remote images.
    /// 
    /// # Arguments
    /// * `venue_path` - The path to the venue where all dependencies are stored
    /// * `name` - The name of the repository where we can run the remote images
    pub fn get_remote_compose_files(&self, venue_path: &String, name: &String) -> String {
        let invite_path = Path::new(&venue_path).join(&name).to_string_lossy().to_string();
        let mut files_string = String::new();
        for file in self.remote_runner_files.as_ref().unwrap() {
            files_string.push_str(&format!("-f {}/{} ", &invite_path, file));
        }
        files_string
    }
}


#[cfg(test)]
mod local_data_tests {
    
    use super::*;
    use crate::file_handler::MockCoreFileHandle;
    use mockall::predicate::eq;

    #[test]
    fn test_from_file() {
        let mut normal_builds = HashMap::new();
        normal_builds.insert("x86_64".to_string(), "build/Dockerfile.x86_64".to_string());
        normal_builds.insert("aarch64".to_string(), "build/Dockerfile.aarch64".to_string());

        let mut init_builds = HashMap::new();
        init_builds.insert("x86_64".to_string(), "database/build/Dockerfile.init".to_string());
        init_builds.insert("aarch64".to_string(), "database/build/Dockerfile.init.arch".to_string());

        let ld = WeddingInvite::from_file("./tests/test_repo/wedding_invite.yml".to_string()).unwrap();
        assert_eq!(ld.build_files, Some(normal_builds));

        assert_eq!(ld.build_root, ".");
        assert_eq!(ld.init_build, Some(InitBuild {
            build_files: init_builds,
            build_root: "database".to_string(),
            build_lock: None
        }));
    }

    #[test]
    fn test_from_file_missing() {
        let ld = WeddingInvite::from_file("./tests/wedding_invite_missing.yml".to_string());
        assert!(ld.is_err());
    }

    #[test]
    fn test_prepare_build_file() {

        let mut normal_builds = HashMap::new();
        normal_builds.insert("x86_64".to_string(), "build/Dockerfile.aarch64".to_string());
        normal_builds.insert("aarch64".to_string(), "build/Dockerfile.aarch64".to_string());

        let mut wedding_invite = WeddingInvite::from_file("./tests/test_repo/wedding_invite.yml".to_string()).unwrap();
        wedding_invite.build_files = Some(normal_builds);

        let mut mock_handle = MockCoreFileHandle::new();
        let from_path = Path::new("./tests/test_repo/build/Dockerfile.aarch64");
        let to_path = Path::new("./tests/test_repo/./Dockerfile");

        mock_handle.expect_copy()
            .with(eq(from_path), eq(to_path))
            .returning(|_, _| {
                Ok(0)
            });
        let result = wedding_invite.prepare_build_file(
            &"./tests".to_string(), &"test_repo".to_string(), 
            &mock_handle);
        assert!(result.is_ok());
        mock_handle.checkpoint(); 
    }

    #[test]
    fn test_delete_build_file() {
        let wedding_invite = WeddingInvite::from_file("./tests/test_repo/wedding_invite.yml".to_string()).unwrap();

        let mut mock_handle = MockCoreFileHandle::new();
        let to_path = Path::new("./tests/test_repo/./Dockerfile");

        mock_handle.expect_remove()
            .with(eq(to_path))
            .returning(|_| {
                Ok(())
            });
        let result = wedding_invite.delete_build_file(
            &"./tests".to_string(), &"test_repo".to_string(), 
            &mut mock_handle);
        assert!(result.is_ok());
        mock_handle.checkpoint(); 
    }

    #[test]
    fn test_prepare_init_build_file() {
        let mut normal_builds = HashMap::new();
        normal_builds.insert("x86_64".to_string(), "database/build/Dockerfile.aarch64".to_string());
        normal_builds.insert("aarch64".to_string(), "database/build/Dockerfile.aarch64".to_string());

        let mut wedding_invite = WeddingInvite::from_file("./tests/test_repo/wedding_invite.yml".to_string()).unwrap();
        wedding_invite.init_build = Some(InitBuild {
            build_files: normal_builds,
            build_root: "database".to_string(),
            build_lock: None
        });

        let mut mock_handle = MockCoreFileHandle::new();
        let from_path = Path::new("./tests/test_repo/database/build/Dockerfile.aarch64");
        let to_path = Path::new("./tests/test_repo/database/Dockerfile");

        mock_handle.expect_copy()
            .with(eq(from_path), eq(to_path))
            .returning(|_, _| {
                Ok(0)
            });
        let result = wedding_invite.prepare_init_build_file(
            &"./tests/".to_string(), &"test_repo".to_string(), 
            &mut mock_handle);
        assert!(result.is_ok());
        mock_handle.checkpoint(); 
    }

    #[test]
    fn test_delete_init_build_file() {
        let mut normal_builds = HashMap::new();
        normal_builds.insert("x86_64".to_string(), "database/build/Dockerfile.aarch64".to_string());
        normal_builds.insert("aarch64".to_string(), "database/build/Dockerfile.aarch64".to_string());

        let mut wedding_invite = WeddingInvite::from_file("./tests/test_repo/wedding_invite.yml".to_string()).unwrap();
        wedding_invite.init_build = Some(InitBuild {
            build_files: normal_builds,
            build_root: "database".to_string(),
            build_lock: None
        });

        let mut mock_handle = MockCoreFileHandle::new();
        let to_path = Path::new("./tests/test_repo/database/Dockerfile");

        mock_handle.expect_remove()
            .with(eq(to_path))
            .returning(|_| {
                Ok(())
            });
        let result = wedding_invite.delete_init_build_file(
            &"./tests/".to_string(), &"test_repo".to_string(), 
            &mut mock_handle);
        assert!(result.is_ok());
        mock_handle.checkpoint(); 
    }

    #[test]
    fn test_get_docker_compose_files() {
        let wedding_invite = WeddingInvite::from_file("./tests/test_repo/wedding_invite.yml".to_string()).unwrap();
        let docker_compose_files = wedding_invite.get_docker_compose_files(&"./tests/".to_string(), &"test_repo".to_string());
        let expected_files = "-f ./tests/test_repo/runner_files/base.yml -f ./tests/test_repo/runner_files/database.yml ".to_string();
        assert_eq!(docker_compose_files, expected_files);
    }
}