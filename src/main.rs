use clap::Parser;

mod options;
mod previewer;
mod result;
mod utils;

fn main() {
    let mut stdout = std::io::stdout();
    let options = options::Options::parse();

    match previewer::preview(&mut stdout, &options) {
        Ok(()) => {}
        Err(err) => eprintln!("{err}"),
    };
}
