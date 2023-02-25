//! Runs the seating plan and the wedding invite of the repo running wedding planner.
use crate::runner::Runner;
use crate::wedding_invite::WeddingInvite;
use crate::file_handler::FileHandle;
use crate::commands::command_runner::{CommandRunner, CoreRunner};


/// constructs the ```DressRehearsal``` struct and runs the command passed in.
/// 
/// # Arguments
/// * `command` - The command to run
/// * `seating_plan_path` - The path to the seating plan file
/// * `wedding_invite_path` - The path to the wedding invite file
/// * `working_directory` - The path to the working directory
pub fn dress_rehearsal_factory(command: String, seating_plan_path: String, wedding_invite_path: String, working_directory: String) {
    let file_handle = FileHandle{};

    let dress_rehearsal = match DressRehearsal::new(seating_plan_path.clone(), wedding_invite_path.clone(), &working_directory) {
        Ok(dress_rehearsal) => dress_rehearsal,
        Err(error) => {
            println!("{} for seating plan path: {} wedding invite path: {} working dir {}", error, seating_plan_path, wedding_invite_path, working_directory);
            return;
        }
    };
    match command.as_ref() {

        "dressbuild" => {
            match dress_rehearsal.wedding_invite.prepare_build_file(&working_directory, &"".to_string(), &file_handle) {
                Ok(_) => {
                    println!("local wedding invite prepared build")
                },
                Err(error) => {
                    println!("local wedding invite failed to prepare build: {}", error);
                }
            };
            match dress_rehearsal.wedding_invite.prepare_init_build_file(&working_directory, &"".to_string(), &file_handle) {
                Ok(_) => {
                    println!("local wedding invite prepared init build")
                },
                Err(error) => {
                    println!("local wedding invite failed to prepare init build: {}", error);
                }
            };
            dress_rehearsal.build_dependencies();
        },
        "dressremotebuild" => {
            match dress_rehearsal.wedding_invite.prepare_build_file(&working_directory, &"".to_string(), &file_handle) {
                Ok(_) => {
                    println!("local wedding invite prepared build")
                },
                Err(error) => {
                    println!("local wedding invite failed to prepare build: {}", error);
                }
            };
            match dress_rehearsal.wedding_invite.prepare_init_build_file(&working_directory, &"".to_string(), &file_handle) {
                Ok(_) => {
                    println!("local wedding invite prepared init build")
                },
                Err(error) => {
                    println!("local wedding invite failed to prepare init build: {}", error);
                }
            };
            dress_rehearsal.build_remote_dependencies();
        },
        "dressrun" => {
            dress_rehearsal.run_dependencies();
        },
        "dressdevrun" => {
            dress_rehearsal.run_dev_dependencies();
        },
        "dressrun-d" => {
            dress_rehearsal.run_dependencies_background();
        },
        "dressremoterun" => {
            dress_rehearsal.run_remote_dependencies();
        },
        "dressremoterun-d" => {
            dress_rehearsal.run_remote_dependencies_background();
        },
        "dressinstall" => {
            dress_rehearsal.runner.install_dependencies();
        },
        "dressteardown" => {
            dress_rehearsal.teardown_dependencies();
        },
        "dressremoteteardown" => {
            dress_rehearsal.teardown_remote_dependencies();
            match dress_rehearsal.wedding_invite.delete_build_file(&working_directory, &"".to_string(), &file_handle){
                Ok(_) => {
                    println!("local wedding invite deleted build")
                },
                Err(error) => {
                    println!("local wedding invite failed to delete build: {}", error);
                }
            };
            match dress_rehearsal.wedding_invite.delete_init_build_file(&working_directory, &"".to_string(), &file_handle) {
                Ok(_) => {
                    println!("local wedding invite deleted init build")
                },
                Err(error) => {
                    println!("local wedding invite failed to delete init build: {}", error);
                }
            };
        },
        "dresssetup" => {
            dress_rehearsal.runner.create_venue();
        }
        _ => {
            println!("{} not supported", command);
        }
    }

}


/// The struct that holds the seating plan and the wedding invite to run.
/// 
/// # Fields
/// * `runner` - The runner that runs the seating plan
/// * `wedding_invite` - The wedding invite that defines build for the repo running wedding planner
/// * `working_directory` - The working directory of the repo running local invite docker files
pub struct DressRehearsal {
    pub runner: Runner,
    pub wedding_invite: WeddingInvite,
    pub working_directory: String
}

impl DressRehearsal {

    /// The constructor for the DressRehearsal struct.
    /// 
    /// # Arguments
    /// * `seating_plan_path` - The path to the seating plan file for the repo running wedding planner
    /// * `wedding_invite_path` - The path to the wedding invite file for the repo running wedding planner
    /// * `working_directory` - The working directory of the repo running local invite docker files
    /// 
    /// # Returns
    /// * `Result<DressRehearsal, String>` - The DressRehearsal struct or an error message
    pub fn new(seating_plan_path: String, wedding_invite_path: String, working_directory: &String) -> Result<DressRehearsal, String> {
        let runner = match Runner::new(seating_plan_path){
            Ok(runner) => runner,
            Err(error) => return Err(error)
        };
        let wedding_invite = match WeddingInvite::from_file(wedding_invite_path) {
            Ok(wedding_invite) => wedding_invite,
            Err(error) => return Err(error)
        };
        Ok(DressRehearsal{runner, wedding_invite, working_directory: working_directory.clone()})
    }

    /// Gets the docker-compose command for the dependencies in the seating plan and local wedding invite.
    /// 
    /// # Arguments
    /// * `remote` - Whether the command is for remote dependencies
    /// 
    /// # Returns
    /// * `String` - The docker-compose command
    fn get_compose_file_command(&self, remote: bool) -> String {
        let mut command_string = self.runner.get_compose_file_command(remote);

        for file in &self.wedding_invite.runner_files {
            command_string.push_str(&format!("-f {}/{} ", self.working_directory, file));
        }
        return command_string;
    }

    /// Gets the docker-compose command for the dependencies in the seating plan and local wedding invite for dev mode.
    /// 
    /// # Returns
    /// * `String` - The docker-compose command
    fn get_compose_file_command_dev(&self) -> String {
        let mut command_string = self.runner.get_compose_file_command(false);

        match &self.wedding_invite.dev_runner_files {
            Some(dev_runner_files) => {
                for file in dev_runner_files {
                    command_string.push_str(&format!("-f {}/{} ", self.working_directory, file));
                }
            },
            None => {}
        }
        return command_string;
    }

    /// Tears down the dependencies that are running.
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose down command for each file
    pub fn teardown_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" down", "failed to tear down", &mut command_string);
    }

    /// Tears down the remote dependencies that are running.
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the remote_runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose down command for each file
    pub fn teardown_remote_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(true);
        command_runner.run_docker_command(" down", "failed to tear down", &mut command_string);
    }

    /// Builds the dependencies that are needed to run. 
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose build command for each file
    pub fn build_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" build --no-cache", "failed to build", &mut command_string);
    }

    /// Builds the remote dependencies.
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the remote_runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose build command for each file
    pub fn build_remote_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(true);
        command_runner.run_docker_command(" build --no-cache", "failed to build remote dependencies", &mut command_string);
    }

    /// Runs the dependencies defined.
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose up command for each file
    pub fn run_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" up", "failed to run dependencies", &mut command_string);
    }

    /// Runs the dependencies defined in the background.
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose up -d command for each file
    pub fn run_dependencies_background(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(false);
        command_runner.run_docker_command(" up -d", "failed to run dependencies in the background", &mut command_string);
    }

    /// Runs the remote dependencies defined.
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the remote_runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose up command for each file
    pub fn run_remote_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(true);
        command_runner.run_docker_command(" up", "failed to run remote dependencies", &mut command_string);
    }

    /// Runs the remote dependencies defined in the background.
    /// 
    /// # Process
    /// 1. Gets all the runner_files from the local wedding invite and the remote_runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose up -d command for each file
    pub fn run_remote_dependencies_background(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command(true);
        command_runner.run_docker_command(" up -d", "failed to run remote dependencies in the background", &mut command_string);
    }

    /// Runs the dependencies defined in dev mode.
    /// 
    /// # Process
    /// 1. Gets all the dev_runner_files from the local wedding invite and the runner_files from the wedding_invite of each dependency
    /// 2. Runs the docker-compose up command for each file
    pub fn run_dev_dependencies(&self) {
        let command_runner = CommandRunner {};
        let mut command_string = self.get_compose_file_command_dev();
        command_runner.run_docker_command(" up", "failed to run dependencies in dev mode", &mut command_string);
    }
}
