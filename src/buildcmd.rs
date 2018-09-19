use error::Error;
use std::process::Command;

#[derive(StructOpt)]
pub struct BuildCommand {
    /// Build artifacts in release mode, with optimizations
    #[structopt(long = "release")]
    release: bool,
}

impl BuildCommand {
    pub fn exec(&self) -> Result<(), Error> {
        let mut cargo_build = "cargo build --target wasm32-unknown-unknown".to_string();
        if self.release {
            cargo_build.push_str(" --release");
        }

        let child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cargo_build])
                .spawn()
                .map_err(Error::BuildFailed)
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cargo_build)
                .spawn()
                .map_err(Error::BuildFailed)
        };
        child?.wait().map_err(Error::BuildFailed)?;
        Ok(())
    }
}
