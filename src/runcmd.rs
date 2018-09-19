#[derive(StructOpt)]
pub struct RunCommand {
    /// Build and run artifacts in release mode, with optimizations
    #[structopt(long = "release")]
    release: bool,
}
