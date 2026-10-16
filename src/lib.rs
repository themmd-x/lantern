pub mod detect;
pub mod command;
pub enum Mode {
    Run,
    Build,
    Check,
    Clean,
    RunAndBuild,
}
