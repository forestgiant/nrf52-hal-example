# NRF52832 Example
This is the first program from the [rust-embedded discovery book](https://rust-embedded.github.io/discovery/index.html) running on an nRF52832 board

## Status
- Currently building examples project is in the very early stages

## Pre-reqs
- ARM toolchain
- [J-Link](https://www.segger.com/downloads/jlink)
- [newlib](ftp://sourceware.org/pub/newlib/index.html)
  - On some macOS you might need to install `newlib` manually
  - Download the above and copy the `newlib` directory into the `/usr/local/include` folder on your mac
  - If you decide to place it somewhere else make sure you update the clang_arg in the [build.rs](build.rs)

## Running
- Start jlink `./jlink`
- Pick and example and run it, e.g. `cargo run --example hello`
- It is setup to break at the first line of code inside the main


## Examples
- [hello](examples/hello.rs)
- [blinky](examples/blinky.rs)
