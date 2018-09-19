use buildcmd::BuildCommand;
use colored::Colorize;
use error::Error;
use notify::{self, RecursiveMode, Watcher};
use parse::CliData;
use server::launch_server;
use std::{path::Path, sync::mpsc, thread, time::Duration};

#[derive(StructOpt)]
pub struct RunCommand {
    /// Build and run artifacts in release mode, with optimizations
    #[structopt(long = "release")]
    pub release: bool,
    /// Run project with rebuilds on file change
    #[structopt(long = "watch")]
    pub watch: bool,
}

impl RunCommand {
    pub fn exec(&self) -> Result<(), Error> {
        let buildcmd = BuildCommand {
            release: self.release,
        };
        buildcmd.exec()?;

        let cli_data = CliData::sniff()?;
        let is_debug = !self.release;
        let cli_data2 = cli_data.clone();
        let server_thread = thread::spawn(move || launch_server(is_debug, cli_data2));

        if self.watch {
            let watch_path = cli_data.project_path().join("src");
            println!(
                "    {} {}",
                "Watching".green().bold(),
                watch_path.to_string_lossy()
            );
            Self::watch_files(&buildcmd, &watch_path);
        }

        server_thread.join().unwrap()?;
        Ok(())
    }

    fn watch_files(buildcmd: &BuildCommand, watch_dir: &Path) {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::watcher(tx, Duration::from_millis(100)).unwrap();
        watcher.watch(watch_dir, RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(_) => if let Err(err) = buildcmd.exec() {
                    eprintln!("{}", err);
                },
                Err(err) => eprintln!("{}", err),
            };
        }
    }
}
