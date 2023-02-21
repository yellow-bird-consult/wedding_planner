//! This tool is used for installing the  dependencies. 
//! 
//! # Uses
//! We can create the venue with the following command: 
//! ```bash
//! wedp setup -f tests/live_test.yml
//! ```
//! We can install the dependencies with the following command: 
//! ```bash
//! wedp install -f tests/live_test.yml
//! ```
//! We can run the dependencies with the following command: 
//! ```bash
//! wedp run -f tests/live_test.yml
//! ```
//! We can teardown the dependency containers with the following command: 
//! ```bash
//! wedp teardown -f tests/live_test.yml
//! ```
use clap::{App, Arg};

use std::{env, path::Path};

mod cpu_data;
mod dependency;
mod seating_plan;
mod wedding_invite;
mod runner;
mod dress_rehearsal;

use runner::Runner;
use dress_rehearsal::dress_rehearsal_factory;


fn main() {
    let matches = App::new("wedding planner")
        .version("0.1.0")
        .author("Maxwell Flitton <maxwellflitton@gmail.com>")
        .about("Basic tool for running docker builds from other Github repos")
        .arg(
            Arg::with_name("command")
                .value_name("COMMAND")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Input command")
        )
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .short("f")
                .long("file")
                .help("Optional file argument")
        )
        .get_matches();

    let cwd = env::current_dir().unwrap().to_str().unwrap().to_owned();
    let command = &matches.values_of_lossy("command").unwrap()[0];
    let file_name = match &matches.values_of_lossy("file"){
        Some(file_name) => file_name[0].clone(),
        None => "wedding_planner.yml".to_owned()
    };
    let full_file_path = Path::new(&cwd).join(&file_name).as_os_str().to_str().unwrap().to_owned();

    match command.as_ref() {

        "build" => {
            match Runner::new(full_file_path) {
                Ok(runner) => runner.build_dependencies(),
                Err(error) => println!("{}", error)
            }
        },
        "run" => {
            match Runner::new(full_file_path) {
                Ok(runner) => runner.run_dependencies(),
                Err(error) => println!("{}", error)
            }
        },
        "remoterun" => {
            match Runner::new(full_file_path) {
                Ok(runner) => runner.run_remote_dependencies(),
                Err(error) => println!("{}", error)
            }
        },
        "install" => {
            match Runner::new(full_file_path) {
                Ok(runner) => runner.install_dependencies(),
                Err(error) => println!("{}", error)
            }
        },
        "teardown" => {
            match Runner::new(full_file_path) {
                Ok(runner) => runner.teardown_dependencies(),
                Err(error) => println!("{}", error)
            }
        },
        "remoteteardown" => {
            match Runner::new(full_file_path) {
                Ok(runner) => runner.teardown_remote_dependencies(),
                Err(error) => println!("{}", error)
            }
        },
        "setup" => {
            match Runner::new(full_file_path) {
                Ok(runner) => runner.create_venue(),
                Err(error) => println!("{}", error)
            }
        }
        _ => {
            let seating_plan_path = "".to_owned();
            let wedding_invite_path = "".to_owned();
            dress_rehearsal_factory(command.to_string(), seating_plan_path, wedding_invite_path, cwd);
        }
    }
}


// test integration
#[cfg(test)]
mod main_tests {

    use assert_cmd::Command;
    use predicates::prelude::*;
    // use std::fs;

    // #[test]
    // fn dies_no_args() {
    //     let mut cmd = Command::cargo_bin("wedp").unwrap();
    //     cmd.assert()
    //         .failure()
    //         .stderr(predicate::str::contains("USAGE"));
    // }

    #[test]
    fn runs() {
        let mut cmd = Command::cargo_bin("wedp").unwrap();
        cmd.arg("hello").assert().success();
    }
    //
    // #[test]
    // fn hello1() {
    //     // let outfile = "./hello1.txt";
    //     // let expected = fs::read_to_string(outfile).unwrap();
    //     let mut cmd = Command::cargo_bin("wedp").unwrap();
    //     cmd.arg("Hello there").assert().success().stdout("Hello there\n\n");
    // }
    //
    // #[test]
    // fn hello2() {
    //     // let outfile = "./hello1.txt";
    //     // let expected = fs::read_to_string(outfile).unwrap();
    //     let mut cmd = Command::cargo_bin("wedp").unwrap();
    //     cmd.args(vec!["Hello", "there"]).assert().success().stdout("Hello there\n\n");
    // }

}