# robco
A command-line tool to assist the hacking of RobCo terminals in case of forgotten password. Please use ethically.

## Setup
Robco is written in Rust 1.49.0, so make sure that is installed.

## To run
Inside the project folder, run `cargo run [password guesses]` to run in debug mode, and `cargo run --release [password guesses]` to run with optimisations on.

To extract the executable, run `cargo build --release`, and copy the executable from `./target/release/robco`.

## Usage
First you must enter the list of possible passwords. There are two ways of doing this. The first is to enter each one as a command-line argument. (TODO: example). If no arguments are given, enter each password on a single line. When all passwords are entered, hacking can begin.

+ To see the list of available passwords, use the `view` command.
+ Once you've made a guess at your RobCo terminal and seen how many characters are correct, use the `guess` command, following the format `guess <password> <correctness>`. This filters out all passwords from the list that can be eliminated. Once there is only one password left, the robco will print it and exit.
+ To see the guess that robco recommends, use the `recommend` command.
+ To exit the program prematurely, use the `exit` command. (NOTE: this doesn't actually work just yet. For now, end the program with CTRL+C or however your shell does it.)

## Example
TODO
