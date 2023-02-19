use clap::{App, Arg};

use std::{env, path::Path};

mod cpu_data;
mod dependency;
mod seating_plan;
mod wedding_invite;


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
            println!("{} supported", command);
            let seating_plan = seating_plan::SeatingPlan::from_file(full_file_path);
            let mut command = "docker-compose ".to_owned();
            match seating_plan {
                Ok(seating_plan) => {
                    let venue = seating_plan.venue;

                    for dependency in &seating_plan.attendees {
                        dependency.clone_github_repo(&venue);
                        dependency.checkout_branch(&venue);
                        let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();
                        wedding_invite.prepare_build_file(&venue, &dependency.name);
                        wedding_invite.prepare_init_build_file(&venue, &dependency.name);
                        let files = &wedding_invite.get_docker_compose_files(&venue, &dependency.name);
                        command.push_str(files);
                    }
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        
        },
        "run" => {
            println!("running the docker-compose");
            let seating_plan = seating_plan::SeatingPlan::from_file(full_file_path);
            let mut command = "docker-compose ".to_owned();
            match seating_plan {
                Ok(seating_plan) => {
                    let venue = seating_plan.venue;

                    for dependency in &seating_plan.attendees {
                        let wedding_invite = dependency.get_wedding_invite(&venue).unwrap();
                        let files = &wedding_invite.get_docker_compose_files(&venue, &dependency.name);
                        command.push_str(files);
                    }
                    command.push_str("up");
                    println!("{}", command);
                    let _ = std::process::Command::new("bash")
                                                    .arg("-c")
                                                    .arg(command)
                                                    .output()
                                                    .expect("failed to run");
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
        _ => {
            println!("{} not supported", command);
        }

    };

    // let omit_newline = matches.is_present("omit_newline");



    // let cwd = env::current_dir().unwrap().to_str().unwrap().to_owned();
    // println!("Current directory: {}", cwd);

    // let mut ending = "\n";
    // if omit_newline {
    //     ending = "";
    // }

    // println!("{}{}", text.join(" "), ending);
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