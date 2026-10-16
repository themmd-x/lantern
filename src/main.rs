use std::{env, path::Path, process};
use lantern::{Mode, command, detect};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode: Mode = match args.get(1).map(String::as_str) {
        Some("build") => Mode::Build,
        Some("runc") => Mode::Run,
        Some("check") => Mode::Check,
        Some("clean") => Mode::Clean,
        Some("run") => Mode::RunAndBuild,

        Some(_) | None => {
            eprintln!("Invalid or missing mode");
            eprintln!("Usage: lantern <build|runc|check|clean|run>");
            process::exit(1);
        }
    };

    for project in detect::detect(
        Path::new("."),
        &detect::project_evidence(),
    ) {
        println!("Found: {}", project.name);
        match mode {
            Mode::Build => _ = command::run_command(&project.build_cmd),
            Mode::Run => _ = command::run_command(&project.run_cmd),
            Mode::Check => _ = command::run_command(&project.check_cmd),
            Mode::Clean => _ = command::run_command(&project.clean_cmd),
            Mode::RunAndBuild => {
                _ = command::run_command(&project.build_cmd);
                _ = command::run_command(&project.run_cmd);
            },
        }
    }
}
