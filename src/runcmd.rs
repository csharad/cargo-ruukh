use buildcmd::BuildCommand;
use error::Error;
use server::launch_server;

#[derive(StructOpt)]
pub struct RunCommand {
    /// Build and run artifacts in release mode, with optimizations
    #[structopt(long = "release")]
    pub release: bool,
}

impl RunCommand {
    pub fn exec(&self) -> Result<(), Error> {
        let buildcmd = BuildCommand {
            release: self.release,
        };
        buildcmd.exec()?;
        launch_server(!self.release)
    }
}
