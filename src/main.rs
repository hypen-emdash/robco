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
        eprintln!("{}", e);
    }
}

fn run(opt: Opt) -> anyhow::Result<()> {
    let hacker = match Hacker::new(opt.passwords) {
        Some(hacker) => hacker,
        None => build_hacker()?,
    };

    let user = TextStreamUser::std();

    let mut app = App { hacker, user };

    app.run()?;
    Ok(())
}

fn build_hacker() -> anyhow::Result<Hacker> {
    eprintln!("Enter candidate passwords. End with blank line.");
    let stdin = io::stdin();

    let mut all_passwords = Vec::new();
    loop {
        eprint!("> ");
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let password = input.trim();
        if password.is_empty() && !all_passwords.is_empty() {
            break;
        } else {
            all_passwords.push(password.to_owned());
        }
    }
    eprintln!("Candidate passwords accepted.");

    let hacker =
        Hacker::new(all_passwords).expect("We don't exit the loop until `passwords` is not empty.");
    Ok(hacker)
}
