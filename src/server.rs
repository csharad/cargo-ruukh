use colored::Colorize;
use error::Error;
use parse::CliData;
use std::fs;
use warp::{self, Filter};

pub fn launch_server(debug: bool, cli_data: CliData) -> Result<(), Error> {
    let package_name = cli_data.package_name.replace('-', "_");
    let html = if let Some(ref index_html_path) = cli_data.index_html_path {
        fs::read_to_string(index_html_path)
            .map_err(Error::Io)?
            .replace("$PACKAGE_NAME_JS$", &format!("{}.js", package_name))
            .replace("$PACKAGE_NAME_WASM$", &format!("{}_bg.wasm", package_name))
    } else {
        format!("
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\"
          content=\"width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">
    <title>{package_name}</title>
</head>
<body>
    <div id=\"app\"></div>
    <script src=\"build/{package_name}.js\"></script>
    <script>
        wasm_bindgen(\"build/{package_name}_bg.wasm\").then(() => wasm_bindgen.run());
    </script>
</body>
</html>
        ", package_name = package_name)
    };

    let build_dir = warp::path("build").and(warp::fs::dir(cli_data.target_path(debug)));

    let any_page = warp::any().map(move || html.clone());
    let routes = build_dir.or(any_page);

    println!("     {} at http://localhost:3000", "Serving".green().bold());
    warp::serve(routes).run(([127, 0, 0, 1], 3000));
    Ok(())
}
