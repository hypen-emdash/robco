use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Hacker {
    /// All the strings that could conceivably be passwords to the terminal.
    /// Must not have duplicates.
    passwords: Vec<String>,
}

#[derive(Debug, Error)]
pub enum FilterError {
    #[error("\"{0}\" is not in the list of available passwords.")]
    UnknownPassword(String),
    #[error("No possible passwords would remain.")]
    Impossible,
    #[error("\"{0}\" cannot have {1} characters correct.")]
    InvalidCorrectness(String, usize),
}

impl Hacker {
    /// Creates a new hacker given a list of candidate passwords.
    /// The list must be nonempty - returns `None` if the list is empty.
    pub fn new(mut passwords: Vec<String>) -> Option<Self> {
        if passwords.is_empty() {
            None
        } else {
            passwords.sort_unstable();
            passwords.dedup();

            Some(Self { passwords })
        }
    }

    /// Filters out all passwords that don't share a correctness with `guess`.
    /// In case of error, does nothing and returns that error.
    pub fn filter(&mut self, guess: &str, correctness: usize) -> Result<(), FilterError> {
        if !self.passwords.iter().any(|pw| pw == guess) {
            // The guess must be from the list.
            Err(FilterError::UnknownPassword(guess.to_owned()))
        } else if correctness > guess.chars().count() {
            // A guess cannot have a higher correctness count than its length.
            Err(FilterError::InvalidCorrectness(
                guess.to_string(),
                correctness,
            ))
        } else if !self
            .passwords
            .iter()
            .any(|pw| commonality(pw, guess) == correctness)
        {
            // A guess shouldn't filter out *all* remaining passwords.
            Err(FilterError::Impossible)
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
    pub fn answer(&self) -> Option<&str> {
        debug_assert!(!self.passwords.is_empty());
        if self.passwords.len() == 1 {
            Some(&self.passwords[0])
        } else {
            None
        }
    }

    /// Get a list of all strings that could be the password.
    pub fn candidates(&self) -> impl Iterator<Item = &str> {
        self.passwords.iter().map(|pw: &String| pw.as_ref())
    }

    /// Recommend the next password to guess.
    pub fn recommend(&self) -> &str {
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
            .expect("Set of candidates cannot be empty.")
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
