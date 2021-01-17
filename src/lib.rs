pub mod hacker;
pub mod user;

pub use hacker::Hacker;
pub use user::{TextStreamUser, User};

pub struct App<U> {
    pub hacker: Hacker,
    pub user: U,
}

impl<U> App<U>
where
    U: User,
{
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            match self.hacker.answer() {
                Some(answer) => {
                    self.user.show_answer(answer)?;
                    return Ok(());
                }
                None => {
                    self.step();
                }
            };
        }
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        use user::Request;

        let command = self.user.get_request()?;
        match command {
            Request::Exit => {}
            Request::SeePasswords => {
                self.user.show_passwords(self.hacker.candidates())?;
            }
            Request::SeeRecommended => {
                self.user.show_recommended(self.hacker.recommend())?;
            }
            Request::FilterPasswords { guess, correctness } => {
                if let Err(e) = self.hacker.filter(&guess, correctness) {
                    eprintln!("Error: {}", e);
                }
            }
        };

        Ok(())
    }
}
