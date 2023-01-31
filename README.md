# PIC 📷

PIC (**P**review **I**mage in **C**LI) is a lightweight Rust tool to preview images in your terminal!
<br>
With support for various image protocols ([`Kitty`](https://sw.kovidgoyal.net/kitty/graphics-protocol/), [`Sixel`](https://saitoha.github.io/libsixel/), [`iTerm`](https://iterm2.com/documentation-images.html)) it works in several terminals, and can still use Unicode blocks in case your terminal isn't supported.

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
    * `--static` and `--loop` options to interact with GIFs
    * `--protocol` option to choose a protocol
    * `--load` `--display` and `--clear` options to interact with Kitty protocol

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
pic Images/YourFavouriteImage.png --cols 13 ...
```

## Examples

Blocks & Top quality previewing

![demo](examples/blocks.png)
![demo](examples/top_quality.png)

Wide choice of options

![options](examples/options.gif)

Really nice GIFs in iTerm

![iterm](examples/iterm.gif)

And also nice in Blocks

![gotcha](examples/blocks.gif)

## Command line usage

```
Preview Image in CLI.

Usage: pic [OPTIONS] <PATH>

Arguments:
  <PATH>  Image to preview

Options:
  -x, --x <X>                x position (0 is left)
  -y, --y <Y>                y position (0 is top)
  -c, --cols <COLS>          Number of cols to fit the preview in
  -r, --rows <ROWS>          Number of rows to fit the preview in
  -u, --upscale              Upscale image if needed
  -s, --static               Only show first frame of GIFs
  -l, --loop                 Loop GIFs infinitely
  -p, --protocol <PROTOCOL>  Previewing protocol to use [possible values: kitty, sixel, iterm, blocks]
      --load <ID>            Load image with the given id (kitty only)
      --display <ID>         Display image with the given id (kitty only)
      --clear <ID>           Clear image with the given id (0 for all) (kitty only)
  -h, --help                 Print help
  -V, --version              Print version
```

## Notes 

- `Sixel` protocol may require [libsixel](https://github.com/saitoha/libsixel) to be installed
- `iTerm` protocol always loop GIFs, except if `--static` is specified

## Progress

Help would be greatly appreciated

- Documentation
    * [ ] Write a greater README
    * [ ] Make releases/packages (publish on crates.io)
- Protocols support
    * [ ] Preview GIFs with Kitty protocol
    * [x] Preview GIFs with Unicode blocks
    * [ ] Work on handling transparency/GIFs with Sixel protocol (GIFs work but don't render well)
    * [ ] Improve protocol support checking (need to test in various terminal)
- Miscellaneous
    * [ ] Implement caching somehow
    * [ ] Show cooler error messages
    * [ ] Write tests (I guess I need to do that...)
