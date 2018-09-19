extern crate toml;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;
extern crate colored;
extern crate warp;

use structopt::StructOpt;

mod buildcmd;
mod error;
mod parse;
mod runcmd;
mod server;

#[derive(StructOpt)]
#[structopt(name = "cargo-ruukh")]
pub enum CargoRuukh {
    /// Compile the current Ruukh project for WASM target
    #[structopt(name = "build")]
    Build(buildcmd::BuildCommand),
    /// Build and execute src/lib.rs on browser
    #[structopt(name = "run")]
    Run(runcmd::RunCommand),
}

impl CargoRuukh {
    pub fn from_args(args: Vec<String>) -> CargoRuukh {
        CargoRuukh::from_iter(args)
    }

    pub fn exec(self) {
        let result = match self {
            CargoRuukh::Build(cmd) => cmd.exec(),
            CargoRuukh::Run(cmd) => cmd.exec(),
        };

        if let Err(err) = result {
            eprintln!("Error occurred: {}", err);
        }
    }
}
