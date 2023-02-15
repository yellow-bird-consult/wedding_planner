use clap::{App, Arg};

use std::env;

mod cpu_data;
mod dependency;
mod seating_plan;
mod wedding_invite;


fn main() {
    let matches = App::new("wedding planner")
        .version("0.1.0")
        .author("Maxwell Flitton <maxwellflitton@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let cwd = env::current_dir().unwrap().to_str().unwrap().to_owned();
    println!("Current directory: {}", cwd);

    let mut ending = "\n";
    if omit_newline {
        ending = "";
    }

    println!("{}{}", text.join(" "), ending);
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