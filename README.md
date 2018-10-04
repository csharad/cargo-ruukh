# Cargo Ruukh

A cargo subcommand to build & run [Ruukh](https://github.com/csharad/ruukh)
projects.

#### !! THIS IS HIGHLY UNSTABLE !!

## Requirement

[`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html)
is required by this CLI, so install it first if you do not have it already.

```shell
$ rustup target add wasm32-unknown-unknown
$ cargo +nightly install wasm-bindgen-cli
```

## Installation

```shell
$ cargo install cargo-ruukh
```

## Build

Build in debug mode.
```shell
$ cargo ruukh build 
```

Build in release mode.
```shell
$ cargo ruukh build --release 
```

## Run

Run the project in browser.
```shell
$ cargo ruukh run
```

Run the project in browser with watch mode.
```shell
$ cargo ruukh run --watch
```

## Help

To see other options available in this CLI.
```shell
$ cargo ruukh -h
```

## Template

The CLI uses a predefined template of html to run the project in the browser.
You may override this template by placing `index.html` within the project's 
`src` folder.
The template looks like this internally. Override this if you need it.

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport"
          content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Your name</title>
</head>
<body>
    <div id="app"></div>
    <script src="build/$PACKAGE_NAME_JS$"></script>
    <script>
        wasm_bindgen("build/$PACKAGE_NAME_WASM$").then(() => wasm_bindgen.run());
    </script>
</body>
</html>
```

As you can see there are two extrapolation variables provided `$PACKAGE_NAME_JS$` & 
`$PACKAGE_NAME_WASM$`.
