# robco
A command-line tool to assist the hacking of RobCo terminals in case of forgotten password. Please use ethically.

## Setup
Robco is written in Rust 1.49.0, so make sure that is installed.

## To run
Inside the project folder, run `cargo run <password guesses>` to run in debug mode, and `cargo run --release <password guesses>` to run with optimisations on.

To extract the executable, run `cargo build --release`, and copy the executable from `./target/release/robco`.
