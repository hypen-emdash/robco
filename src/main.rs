use structopt::StructOpt;
use thiserror::Error;

use std::io;

#[derive(Debug, StructOpt)]
struct Opt {
    passwords: Vec<String>,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("no passwords given")]
    NoPasswords,
    #[error("no password is possible")]
    Impossible,
    #[error("input/output error: {0:?}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("expected \"[guess] [chars correct]\", found blank line")]
    Blank,
    #[error("missing: number of characters correct in {0}")]
    MissingCorrect(String),
    #[error("malformed correctness count in \"{src}\" - {err}")]
    BadCorrect {
        src: String,
        err: std::num::ParseIntError,
    },
    #[error("unexpected token {0}")]
    UnexpectedToken(String),
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt.passwords) {
        eprintln!("Error: {}", e);
    }
}

fn run(passwords: Vec<String>) -> Result<(), AppError> {
    if passwords.is_empty() {
        Err(AppError::NoPasswords)
    } else {
        let password = hack(passwords)?;
        eprint!("The password is: ");
        println!("{}", password);
        Ok(())
    }
}

fn hack(mut passwords: Vec<String>) -> Result<String, AppError> {
    assert!(!passwords.is_empty());

    let stdin = io::stdin();

    let mut guess = String::new();
    while passwords.len() > 1 {
        print_available(&passwords);

        let (password, chars_correct) = loop {
            eprint!("> ");
            guess.clear();
            stdin.read_line(&mut guess)?;

            match parse_password(&guess) {
                Ok((p, c)) => break (p, c),
                Err(ParseError::Blank) => continue,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    continue;
                }
            };
        };

        passwords.retain(|s| commonality(s, password) == chars_correct);
    }

    passwords.pop().ok_or(AppError::Impossible)
}

fn print_available(passwords: &[String]) {
    eprintln!("Remaining possible passwords:");
    for pw in passwords {
        eprint!(" * ");
        println!("{}", pw);
    }
    println!("")
}

fn parse_password(guess: &str) -> Result<(&str, usize), ParseError> {
    let mut tokens = guess.split_whitespace();
    let guess = tokens.next().ok_or(ParseError::Blank)?;
    let correctness_token = tokens
        .next()
        .ok_or_else(|| ParseError::MissingCorrect(guess.to_owned()))?;
    let correctness = correctness_token
        .parse::<usize>()
        .map_err(|err| ParseError::BadCorrect {
            src: correctness_token.to_owned(),
            err,
        })?;

    if let Some(extra) = tokens.next() {
        Err(ParseError::UnexpectedToken(extra.to_owned()))
    } else {
        Ok((guess, correctness))
    }
}

/// Returns the number of characters one string has in common with another.
/// For a character to be common to both strings, it must appear in the same place.
fn commonality(s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .map(|(sc, tc)| usize::from(sc == tc))
        .sum()
}
