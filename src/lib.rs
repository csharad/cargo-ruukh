extern crate toml;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

mod buildcmd;
mod error;
mod parse;
mod runcmd;

#[derive(StructOpt)]
#[structopt(name = "cargo-ruukh")]
pub enum CargoRuukh {
    /// Compile the current Ruukh project for WASM target
    Build(buildcmd::BuildCommand),
    /// Build and execute src/lib.rs on browser
    Run(runcmd::RunCommand),
}
