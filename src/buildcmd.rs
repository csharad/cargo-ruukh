use colored::Colorize;
use error::Error;
use parse::CliData;
use std::io;
use std::process::{Child, Command};

#[derive(StructOpt)]
pub struct BuildCommand {
    /// Build artifacts in release mode, with optimizations
    #[structopt(long = "release")]
    pub release: bool,
}

impl BuildCommand {
    pub fn exec(&self) -> Result<(), Error> {
        println!(
            "    {} Ruukh project for WASM target",
            "Building".green().bold()
        );

        let mut cargo_build = "cargo build --target wasm32-unknown-unknown".to_string();
        if self.release {
            cargo_build.push_str(" --release");
        }

        let mut child = exec_cmd(&cargo_build).map_err(Error::BuildFailed)?;
        child.wait().map_err(Error::BuildFailed)?;

        println!("  {} JS bindings", "Generating".green().bold());
        let cli_data = CliData::sniff()?;
        let target_path = cli_data.wasm_file_path(!self.release);
        let wasm_bindgen = format!(
            "wasm-bindgen --no-modules {} --out-dir {}",
            target_path.to_string_lossy(),
            target_path.parent().unwrap().to_string_lossy()
        );
        match exec_cmd(&wasm_bindgen) {
            Ok(mut child) => {
                child.wait().map_err(Error::BuildFailed)?;
                Ok(())
            }
            Err(e) => if let io::ErrorKind::NotFound = e.kind() {
                Err(Error::WasmBindgenRequired)
            } else {
                Err(Error::BuildFailed(e))
            },
        }
    }
}

fn exec_cmd(arg: &str) -> io::Result<Child> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", arg]).spawn()
    } else {
        Command::new("sh").arg("-c").arg(arg).spawn()
    }
}
