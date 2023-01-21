use clap::Parser;

mod options;
mod previewer;
mod utils;

fn main() {
    let mut stdout = std::io::stdout();
    let options = options::Options::parse();

    match previewer::preview(&mut stdout, &options) {
        Ok(()) => {}
        Err(why) => eprintln!("Error: {why}"),
    };
}
