pub mod hacker;
pub mod user;

pub use hacker::Hacker;
pub use user::{TextStreamUser, User};

pub struct App<U> {
    pub hacker: Hacker,
    pub user: U,
}

struct Terminate(bool);

impl<U> App<U>
where
    U: User,
{
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if let Terminate(true) = self.step()? {
                return Ok(());
            }
        }
    }

    /// If there are no errors, returns whether or not we should terminate the program.
    fn step(&mut self) -> anyhow::Result<Terminate> {
        use user::Request;

        let command = self.user.get_request()?;
        match command {
            Request::Exit => return Ok(Terminate(true)),
            Request::SeePasswords => {
                self.user.show_passwords(self.hacker.candidates())?;
            }
            Request::SeeRecommended => {
                if let Err(e) = self.hacker.recommend() {
                    self.user.show_error(e)?;
                }
            }
            Request::FilterPasswords { guess, correctness } => {
                if let Err(e) = self.hacker.filter(&guess, correctness) {
                    self.user.show_error(e)?;
                }
            }
        };

        Ok(Terminate(false))
    }
}
