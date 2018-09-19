extern crate cargo_ruukh;

use cargo_ruukh::CargoRuukh;
use std::env;

fn main() {
    let args = {
        // To allow running both as 'cargo-ruukh' and as 'cargo ruukh'.
        let mut args = env::args();
        let mut filtered_args = Vec::new();
        filtered_args.push(args.next().unwrap());

        match args.next() {
            None => {}
            #[cfg(unix)]
            Some(ref arg) if filtered_args[0].ends_with("cargo-ruukh") && arg == "ruukh" => {}
            #[cfg(windows)]
            Some(ref arg) if filtered_args[0].ends_with("cargo-ruukh.exe") && arg == "ruukh" => {}
            Some(arg) => filtered_args.push(arg),
        }

        filtered_args.extend(args);
        filtered_args
    };
    CargoRuukh::from_args(args).exec();
}
