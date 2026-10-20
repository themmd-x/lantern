use std::{
    env,
    fs,
    path::Path,
    process
};
use lantern::{
    Mode, 
    command, 
    detect,
    detect::Project,
    Config
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = String::from("lantern.toml");
    let cnf = match fs::read_to_string(&config_file) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default_cnf = Config {
                build_cmd: "auto".to_string(),
                run_cmd: "auto".to_string(),
                test_cmd: "auto".to_string(),
                check_cmd: "auto".to_string(),
                clean_cmd: "auto".to_string(),
                build_and_run_cmd: "auto".to_string(),
            };
            let cnf_string = match toml::to_string_pretty(&default_cnf) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to serialize default config: {}", e);
                    process::exit(1);
                }
            };
            _ = fs::write(&config_file, &cnf_string);
            cnf_string
        },
        Err(e) => {
            eprintln!("Error opening config file {}: {}", &config_file, e);
            process::exit(1);
        },
    };
    let config: Option<Config> = match toml::from_str(&cnf) {
        Ok(v) => Some(v),
        Err(_) => None,
    };

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
    let evidence = detect::project_evidence();
    let projects = detect::detect(
        Path::new("."),
        &evidence,    
    );

    println!("Found: {}", 
        projects.iter()
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
    for project in &projects {
        run_mode(&mode, &project, &config);
    }
}

fn run_mode(mode: &Mode, project: &Project, config: &Option<Config>) {
    match mode {
        Mode::Build => {
            let cmd = config
                .as_ref()
                .map(|c| c.build_cmd.as_str())
                .filter(|cmd| *cmd != "auto")
                .unwrap_or(&project.build_cmd);

            _ = command::run_command(&cmd.to_string());
        }

        Mode::Run => {
            let cmd = config
                .as_ref()
                .map(|c| c.run_cmd.as_str())
                .filter(|cmd| *cmd != "auto")
                .unwrap_or(&project.run_cmd);

            _ = command::run_command(&cmd.to_string());
        }

        Mode::Check => {
            let cmd = config
                .as_ref()
                .map(|c| c.check_cmd.as_str())
                .filter(|cmd| *cmd != "auto")
                .unwrap_or(&project.check_cmd);

            _ = command::run_command(&cmd.to_string());
        }

        Mode::Clean => {
            let cmd = config
                .as_ref()
                .map(|c| c.clean_cmd.as_str())
                .filter(|cmd| *cmd != "auto")
                .unwrap_or(&project.clean_cmd);

            _ = command::run_command(&cmd.to_string());
        }

        Mode::RunAndBuild => {
            let cmd = config
                .as_ref()
                .map(|c| c.build_and_run_cmd.as_str())
                .filter(|cmd| *cmd != "auto");

            if let Some(cmd) = cmd {
                _ = command::run_command(&cmd.to_string());
            } else {
                _ = command::run_command(&project.build_cmd);
                _ = command::run_command(&project.run_cmd);
            }
        }
    }
}
