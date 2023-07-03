# exif-metadata-reader

A simple exif metadata reader written in [Rust](https://www.rust-lang.org/) and [WebAssembly](https://webassembly.org/).

## How to Run the Project

To compile the program, go to the root folder and run the following commands:

1. `cargo build`
2. `wasm-pack build --release --target web`

Then, it is necessary to run the program with a web server. A simple choice may be to run a local server with python: `python3 -m http.server`.

**Note**: since the program consists of an exif metadata reader, it is necessary that the uploaded files support and contain [exif metadata](https://en.wikipedia.org/wiki/Exif).

## Contributors

* [gconfalonieri](https://github.com/gconfalonieri)
* [gando91](https://github.com/gando91)
