use std::process::Command;

pub fn run_command(command: &String) {
    let parts = shell_words::split(&*command)
        .expect("Invalid command syntax");

    let Some((program, args)) = parts.split_first() else {
        eprintln!("Error: command cannot be empty");
        return;
    };

    let status = Command::new(program)
        .args(args)
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("Command completed successfully");
        }
        Ok(status) => {
            eprintln!("Command exited with status: {status}");
        }
        Err(err) => {
            eprintln!("Failed to execute command: {err}");
        }
    }
}
