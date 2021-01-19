mod hacker;
mod user;

use std::io;

use structopt::StructOpt;

use robco::hacker::Hacker;
use robco::user::TextStreamUser;
use robco::App;

#[derive(Debug, StructOpt)]
struct Opt {
    passwords: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("Error: {}", e);
    }
}

fn run(opt: Opt) -> anyhow::Result<()> {
    let hacker = build_hacker(opt.passwords)?;

    let user = TextStreamUser::std();

    let mut app = App { hacker, user };

    app.run()?;
    Ok(())
}

fn build_hacker(mut passwords: Vec<String>) -> anyhow::Result<Hacker> {
    // First, make sure that the list of passwords isn't empty.
    if passwords.is_empty() {
        passwords = collect_passwords()?;
    }

    // Second, warn the user about any whitespace in their passwords.
    warn_whitespace(&mut passwords);

    Ok(Hacker::new(passwords).expect("We made sure the list of passwords was not empty."))
}

fn collect_passwords() -> anyhow::Result<Vec<String>> {
    eprintln!("No candidate passwords detected. Enter below. Enter blank line when finished.");

    let mut all_passwords = Vec::new();
    let stdin = io::stdin();

    loop {
        eprint!("> ");
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if all_passwords.is_empty() {
                continue;
            } else {
                break;
            }
        } else {
            all_passwords.push(trimmed.to_owned());
        }
    }

    eprintln!("Passwords collected. Thank you.\n");
    Ok(all_passwords)
}

fn warn_whitespace(passwords: &[String]) {
    for pw in passwords.iter().filter(|s| s.contains(char::is_whitespace)) {
        eprintln!(
            "Warning: passwords with whitespace not yet supported: \"{}\".\n",
            pw
        );
    }
}
