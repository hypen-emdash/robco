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
    let hacker = Hacker::new(opt.passwords);
    let user = TextStreamUser::std();
    let mut app = App { hacker, user };
    app.run()?;
    Ok(())
}
