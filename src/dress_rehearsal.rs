//! Runs the seating plan and the wedding invite of the repo running wedding planner.
use crate::runner::Runner;
use crate::wedding_invite::WeddingInvite;


/// constructs the ```DressRehearsal``` struct and runs the command passed in.
/// 
/// # Arguments
/// * `command` - The command to run
/// * `seating_plan_path` - The path to the seating plan file
/// * `wedding_invite_path` - The path to the wedding invite file
/// * `working_directory` - The path to the working directory
pub fn dress_rehearsal_factory(command: String, seating_plan_path: String, wedding_invite_path: String, working_directory: String) {

    let dress_rehearsal = match DressRehearsal::new(seating_plan_path, wedding_invite_path, &working_directory) {
        Ok(dress_rehearsal) => dress_rehearsal,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    match command.as_ref() {

        "dressbuild" => {
            dress_rehearsal.build_dependencies();
        },
        "dressrun" => {
            dress_rehearsal.run_dependencies();
        },
        "dressremoterun" => {
            dress_rehearsal.run_remote_dependencies();
        },
        "dressinstall" => {
            dress_rehearsal.runner.install_dependencies();
        },
        "dressteardown" => {
            dress_rehearsal.teardown_dependencies();
        },
        "dressremoteteardown" => {
            dress_rehearsal.teardown_remote_dependencies();
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

    /// Tears down the dependencies that are running.
    pub fn teardown_dependencies(&self) {
        let command_string = self.get_compose_file_command(false);
        self.runner.run_docker_command(" down", "failed to tear down", command_string);
    }

    /// Tears down the remote dependencies that are running.
    pub fn teardown_remote_dependencies(&self) {
        let command_string = self.get_compose_file_command(true);
        self.runner.run_docker_command(" down", "failed to tear down", command_string);
    }

    /// Builds the dependencies that are needed to run. 
    pub fn build_dependencies(&self) {
        let command_string = self.get_compose_file_command(false);
        self.runner.run_docker_command(" build --no-cache", "failed to build", command_string);
    }

    /// Runs the dependencies defined.
    pub fn run_dependencies(&self) {
        let command_string = self.get_compose_file_command(false);
        self.runner.run_docker_command(" up", "failed to run", command_string);
    }

    /// Runs the remote dependencies defined.
    pub fn run_remote_dependencies(&self) {
        let command_string = self.get_compose_file_command(true);
        self.runner.run_docker_command(" up", "failed to run", command_string);
    }
}
