# PIC 📷

PIC (**P**review **I**mage in **C**LI) is a lightweight Rust tool to preview images in your terminal!
<br>
With support for various image protocols ([`Kitty`](https://sw.kovidgoyal.net/kitty/graphics-protocol/), [`Sixel`](https://saitoha.github.io/libsixel/), [`iTerm`](https://iterm2.com/documentation-images.html)) it can work in several terminals, and can still use Unicode in case your terminal isn't supported.

## Features

- Choose your favourite protocols
    * Kitty graphics with multiple actions available (`load`/`clear`)
    * Sixel which works in a lot of terminals
    * iTerm which displays GIFs incredibly well
    * Unicode blocks with truecolor/ansi256 support otherwise
- Customization
    * `--x` and `--y` options to choose where to display your image
    * `--cols` and `--rows` options to choose the size of your image (always tries preserving ratio)
    * `--upscale` option to preview image at full wanted size if needed
    * `--force` option to bypass protocol support

## Installation

### From source

Prerequisites
- [Git](https://git-scm.com/downloads)
- [Rust toolchain](https://www.rust-lang.org/tools/install)

Command line instructions
```bash
# Clone the repository
git clone https://github.com/emsquid/pic

# Build and install
cargo install --path pic

# Use freely
pic kitty display Images/YourFavouriteImage.png --cols 13 ...
```

## Command line usage

```
Preview Images in CLI.

Usage: pic [OPTIONS] <PROTOCOL> <ACTION> <PATH>

Arguments:
  <PROTOCOL>  Previewing protocol to use [possible values: kitty, sixel, iterm, blocks]
  <ACTION>  What to do with the image [possible values: display, load, load-and-display, clear]
  <PATH>    Path to the image to preview

Options:
  -i, --id <ID>      id to use (kitty only)
  -x, --x <X>        x position (0 is left)
  -y, --y <Y>        y position (0 is top)
  -c, --cols <COLS>  Number of cols to fit the preview in
  -r, --rows <ROWS>  Number of rows to fit the preview in
  -u, --upscale      Upscale image if needed
  -f, --force        Do not check for protocol support
  -h, --help         Print help
  -V, --version      Print version
```

## Progress

Help would be greatly appreciated

- [ ] Write a greater README
- [ ] Make releases/packages (publish on crates.io)
- [ ] Preview GIFs with Kitty protocol/Unicode blocks
- [ ] Work on handling transparency/GIFs with Sixel protocol (GIFs work but don't render well)
- [ ] Improve protocol support checking (need to test in various terminal)
- [ ] Show cooler error messages
- [ ] Write tests (I guess I need to do that...)
