use thiserror::Error;

use std::io::{self, BufRead, BufReader, Stderr, Stdin, Stdout, Write};
use std::{error::Error as StdError, fmt::Display};

/// Abstracts out the interface for testing and actual usage.
pub trait User {
    type Err: 'static + Sync + Send + StdError;

    /// Ask the user what they want to do.
    fn get_request(&mut self) -> Result<Request, Self::Err>;

    /// Show the user all remaining passwords.
    fn show_passwords<'a, Iter>(&mut self, passwords: Iter) -> Result<(), Self::Err>
    where
        Iter: Iterator<Item = &'a str>;

    /// Show the user which password is recommended to try next.
    fn show_recommended(&mut self, recommended: &str) -> Result<(), Self::Err>;

    /// Show the user the correct password.
    fn show_answer(&mut self, answer: &str) -> Result<(), Self::Err>;

    /// Show the user something that went wrong.
    fn show_error<E>(&mut self, err: E) -> Result<(), Self::Err>
    where
        E: StdError + Display;
}

pub enum Request {
    Exit,
    SeePasswords,
    SeeRecommended,
    FilterPasswords { guess: String, correctness: usize },
}

pub struct TextStreamUser<I, O, E> {
    input: I,
    output: O,
    errput: E,
}

impl<I, O, E> TextStreamUser<I, O, E>
where
    I: BufRead,
    O: Write,
    E: Write,
{
    pub fn new(input: I, output: O, errput: E) -> Self {
        Self {
            input,
            output,
            errput,
        }
    }
}

impl TextStreamUser<BufReader<Stdin>, Stdout, Stderr> {
    pub fn std() -> Self {
        Self::new(BufReader::new(io::stdin()), io::stdout(), io::stderr())
    }
}

impl<I, O, E> User for TextStreamUser<I, O, E>
where
    I: BufRead,
    O: Write,
    E: Write,
{
    type Err = io::Error;

    fn get_request(&mut self) -> Result<Request, Self::Err> {
        loop {
            write!(self.errput, "> ")?;
            let mut line = String::new();
            self.input.read_line(&mut line)?;

            match parse_request(&line) {
                Ok(request) => return Ok(request),
                Err(e) => self.show_error(e)?,
            }
        }
    }

    fn show_passwords<'a, Iter>(&mut self, passwords: Iter) -> Result<(), Self::Err>
    where
        Iter: Iterator<Item = &'a str>,
    {
        writeln!(self.errput, "Remaining candidate passwords:")?;
        for pw in passwords {
            writeln!(self.output, " * {}", pw)?;
        }
        writeln!(self.output, "")?;

        Ok(())
    }

    fn show_recommended(&mut self, recommended: &str) -> Result<(), Self::Err> {
        write!(self.errput, "Recommended: ")?;
        writeln!(self.output, "{}", recommended)?;
        Ok(())
    }

    fn show_answer(&mut self, answer: &str) -> Result<(), Self::Err> {
        write!(self.errput, "Password deduced: ")?;
        writeln!(self.output, "{}", answer)?;
        Ok(())
    }

    fn show_error<T>(&mut self, err: T) -> Result<(), Self::Err>
    where
        T: StdError + Display,
    {
        writeln!(self.errput, "Error: {}\n", err)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expected command, found blank line")]
    Blank,
    #[error("command not recognised: {0}")]
    UnrecognisedCommand(String),
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("expected token: <{0}>, found nothing")]
    MissingToken(&'static str),
    #[error("cannot parse correctness value - expected nonnegative integer, found {0} ({1})")]
    MalformedCorrectness(String, std::num::ParseIntError),
}

fn parse_request(line: &str) -> Result<Request, ParseError> {
    let mut tokens = line.split_whitespace();
    let command = tokens.next().ok_or(ParseError::Blank)?;
    match command {
        "view" => parse_view(tokens),
        "guess" => parse_guess(tokens),
        "recommend" => parse_recommend(tokens),
        "exit" => parse_exit(tokens),
        unrecognised => Err(ParseError::UnrecognisedCommand(unrecognised.to_owned())),
    }
}

fn parse_view<'a, I>(mut tokens: I) -> Result<Request, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match tokens.next() {
        None => Ok(Request::SeePasswords),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}

fn parse_guess<'a, I>(mut tokens: I) -> Result<Request, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    let guess = tokens.next().ok_or(ParseError::MissingToken("guess"))?;
    let correctness = tokens
        .next()
        .ok_or(ParseError::MissingToken("correctness"))?;
    let correctness = correctness
        .parse::<usize>()
        .map_err(|e| ParseError::MalformedCorrectness(correctness.to_owned(), e))?;

    Ok(Request::FilterPasswords {
        guess: guess.to_owned(),
        correctness,
    })
}

fn parse_recommend<'a, I>(mut tokens: I) -> Result<Request, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match tokens.next() {
        None => Ok(Request::SeeRecommended),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}

fn parse_exit<'a, I>(mut tokens: I) -> Result<Request, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match tokens.next() {
        None => Ok(Request::Exit),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}
