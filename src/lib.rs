use serde::{Deserialize, Serialize};

pub mod detect;
pub mod command;
pub enum Mode {
    Run,
    Build,
    Check,
    Clean,
    RunAndBuild,
}
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub projects: Vec<detect::Project>,
    pub build_cmd: String,
    pub run_cmd: String,
    pub test_cmd: String,
    pub check_cmd: String,
    pub clean_cmd: String,
    pub build_and_run_cmd: String,
}
