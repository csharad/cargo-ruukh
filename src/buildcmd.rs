#[derive(StructOpt)]
pub struct BuildCommand {
    /// Build artifacts in release mode, with optimizations
    #[structopt(long = "release")]
    release: bool,
}
