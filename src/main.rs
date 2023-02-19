use clap::Parser;

fn main() {
    let mut stdout = std::io::stdout();
    let mut options = pic::options::Options::parse_from(wild::args());

    if let Err(err) = pic::previewer::preview(&mut stdout, &mut options) {
        eprintln!("{err}");
    };
}
