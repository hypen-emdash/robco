use thiserror::Error;

use std::io::{self, BufRead, BufReader, Stderr, Stdin, Stdout, Write};
use std::{error::Error as StdError, fmt::Display};

/// Abstracts out the interface for testing and actual usage.
pub trait User {
    type Err: 'static + Sync + Send + StdError;

    /// Ask the user what they want to do.
    fn get_request(&mut self) -> Result<Command, Self::Err>;

    /// Show the user all remaining passwords.
    fn show_passwords<'a, Iter>(&mut self, passwords: Iter) -> Result<(), Self::Err>
    where
        Iter: ExactSizeIterator<Item = &'a str>;

    /// Show the user which password is recommended to try next.
    fn show_recommended(&mut self, recommended: &str) -> Result<(), Self::Err>;

    /// Show the user the correct password.
    fn show_answer(&mut self, answer: &str) -> Result<(), Self::Err>;

    /// Show the user something that went wrong.
    fn show_error<E>(&mut self, err: E) -> Result<(), Self::Err>
    where
        E: StdError + Display;

    fn show_help(&mut self) -> Result<(), Self::Err>;
}

pub enum Command {
    Exit,
    SeePasswords,
    SeeRecommended,
    SeeAnswer,
    FilterPasswords { guess: String, correctness: usize },
    AddPassword(String),
    RemovePassword(String),
    Help,
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

    fn get_request(&mut self) -> Result<Command, Self::Err> {
        loop {
            write!(self.errput, "> ")?;
            let mut line = String::new();
            self.input.read_line(&mut line)?;

            match parse_command(&line) {
                Ok(request) => return Ok(request),
                Err(e) => self.show_error(e)?,
            }
        }
    }

    fn show_passwords<'a, Iter>(&mut self, passwords: Iter) -> Result<(), Self::Err>
    where
        Iter: ExactSizeIterator<Item = &'a str>,
    {
        writeln!(self.errput, "Remaining candidate passwords: ({})", passwords.len())?;
        for pw in passwords {
            writeln!(self.output, " * {}", pw)?;
        }
        writeln!(self.errput)?;

        Ok(())
    }

    fn show_recommended(&mut self, recommended: &str) -> Result<(), Self::Err> {
        write!(self.errput, "Recommended: ")?;
        writeln!(self.output, "{}", recommended)?;
        writeln!(self.errput)?;
        Ok(())
    }

    fn show_answer(&mut self, answer: &str) -> Result<(), Self::Err> {
        write!(self.errput, "Password deduced: ")?;
        writeln!(self.output, "{}", answer)?;
        writeln!(self.errput)?;
        Ok(())
    }

    fn show_error<T>(&mut self, err: T) -> Result<(), Self::Err>
    where
        T: StdError + Display,
    {
        writeln!(self.errput, "Error: {}", err)?;
        writeln!(self.errput)?;
        Ok(())
    }

    fn show_help(&mut self) -> Result<(), Self::Err> {
        writeln!(self.errput, "Help not yet available.")?;
        writeln!(self.errput)?;
        Ok(())
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

fn parse_command(line: &str) -> Result<Command, ParseError> {
    let mut tokens = line.split_whitespace();
    let command = tokens.next().ok_or(ParseError::Blank)?;
    let args = tokens;
    match command {
        "exit" => parse_exit(args),
        "view" => parse_view(args),
        "recommend" => parse_recommend(args),
        "answer" => parse_answer(args), 
        "guess" => parse_guess(args),
        "add" => parse_add(args),
        "remove" => parse_remove(args),
        "help" => parse_help(args),
        unrecognised => Err(ParseError::UnrecognisedCommand(unrecognised.to_owned())),
    }
}

fn parse_exit<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match args.next() {
        None => Ok(Command::Exit),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}

fn parse_help<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match args.next() {
        None => Ok(Command::Help),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}

fn parse_view<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match args.next() {
        None => Ok(Command::SeePasswords),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}

fn parse_answer<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match args.next() {
        None => Ok(Command::SeeAnswer),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}

fn parse_guess<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    let guess = args.next().ok_or(ParseError::MissingToken("guess"))?;
    let correctness = args
        .next()
        .ok_or(ParseError::MissingToken("correctness"))?;
    let correctness = correctness
        .parse::<usize>()
        .map_err(|e| ParseError::MalformedCorrectness(correctness.to_owned(), e))?;

    Ok(Command::FilterPasswords {
        guess: guess.to_owned(),
        correctness,
    })
}

fn parse_recommend<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match args.next() {
        None => Ok(Command::SeeRecommended),
        Some(tok) => Err(ParseError::UnexpectedToken(tok.to_owned())),
    }
}

fn parse_add<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match args.next() {
        Some(pw) => Ok(Command::AddPassword(pw.to_owned())),
        None => Err(ParseError::MissingToken("password to add"))
    }
}

fn parse_remove<'a, I>(mut args: I) -> Result<Command, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    match args.next() {
        Some(pw) => Ok(Command::RemovePassword(pw.to_owned())),
        None => Err(ParseError::MissingToken("password to remove"))
    }
}
