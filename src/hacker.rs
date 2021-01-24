use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Hacker {
    /// All the strings that could conceivably be passwords to the terminal.
    /// Must not have duplicates.
    passwords: Vec<String>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("\"{0}\" is not in the list of available passwords.")]
    NotPresent(String),
    #[error("cannot add \"{0}\": already present.")]
    AlreadyPresent(String),
    #[error("\"{0}\" cannot have {1} characters correct.")]
    InvalidCorrectness(String, usize),
    #[error("pool is empty.")]
    Empty,
}


impl Hacker {
    /// Creates a new hacker given a list of candidate passwords.
    /// The list must be nonempty - returns `None` if the list is empty.
    pub fn new(mut passwords: Vec<String>) -> Self {
        passwords.sort_unstable();
        passwords.dedup();

        Self { passwords }
    }

    /// Filters out all passwords that don't share a correctness with `guess`.
    /// In case of error, does nothing and returns that error.
    pub fn filter(&mut self, guess: &str, correctness: usize) -> Result<(), Error> {
        if !self.passwords.iter().any(|pw| pw == guess) {
            // The guess must be from the list.
            Err(Error::NotPresent(guess.to_owned()))
        } else if correctness > guess.chars().count() {
            // A guess cannot have a higher correctness count than its length.
            Err(Error::InvalidCorrectness(
                guess.to_string(),
                correctness,
            ))
        } else {
            // No errors.
            // Filter out incorrect passwords.
            self.passwords
                .retain(|pw| commonality(pw, guess) == correctness);
            Ok(())
        }
    }

    /// If the hacker knows the correct password (ie if there is only one candidate left), returns it.
    /// Otherwise, returns `None`.
    pub fn answer(&self) -> Result<&str, Error> {
        if self.passwords.len() == 1 {
            Ok(&self.passwords[0])
        } else {
            Err(Error::Empty)
        }
    }

    /// Get a list of all strings that could be the password.
    pub fn candidates(&self) -> impl Iterator<Item = &str> {
        self.passwords.iter().map(|pw: &String| pw.as_ref())
    }

    /// Recommend the next password to guess.
    /// If the password list is empty, returns error case.
    pub fn recommend(&self) -> Result<&str, Error> {
        // For a given guess, assume that all passwords are equally likely, and
        // take the expected size of the candidate pool.
        // (Actually, the expected size is unnormalised, but since it's always
        // off by the same scale factor, we can ignore that.)
        // Lower means more powerful.
        let filtration_power = |guess| {
            self.candidates()
                .map(|true_password| {
                    let correctness = commonality(true_password, guess);
                    self.candidates()
                        .filter(|candidate| commonality(candidate, guess) == correctness)
                        .count()
                })
                .sum::<usize>()
        };

        // Recommend the candidate password that filters out the most.
        self.candidates()
            .min_by_key(|s| filtration_power(s))
            .ok_or(Error::Empty)
    }

    pub fn add(&mut self, password: String) -> Result<(), Error> {
        // Borrow-checker isn't advanced enough to let us do this with a normal
        // match statement - `None` borrows from the source.
        if self.candidates().find(|s| s == &password).is_none() {
            self.passwords.push(password);
            Ok(())
        } else {
            Err(Error::AlreadyPresent(password))
        }
    }

    pub fn remove(&mut self, password: &str) -> Result<(), Error> {
        let i = self
            .candidates()
            .position(|s| s == password)
            .ok_or_else(|| Error::NotPresent(password.to_owned()))?;
        self.passwords.swap_remove(i);
        Ok(())
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
