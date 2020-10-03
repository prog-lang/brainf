# The Brainfuck Interpreter

Brainf interpreter written in Rust. Fast, reliable, and easy to install.



## Installation

If you don't have [Rust](https://rust-lang.org/) yet, install it through
`rustup` as specified [here](https://www.rust-lang.org/tools/install). This
also install `cargo` - Rust's default package manager which we'll use to
install `brainf-rs` in seconds.

Here's what you need to do once you have Rust and Cargo installed:


### Install From `crates.io`

Cargo will simply fetch, compile, and install all files for you. Just make sure
that your `$HOME/.cargo/bin/` folder is in `$PATH`.

```bash
cargo install brainf
```


### Compile From Source

If you wish to tweak some of the code and maybe even contribute, you will
probably fancy having the source code on your machine.

```bash
git clone https://github.com/sharpvik/brainf-rs

cd brainf-rs

cargo install --path .
#     ^ Installs Brainf as executable program on your system.
# If you want to tweak or develop Brainf, you don't have to run this one.
# Try running this instead:

cargo check
cargo build
```



## Contributing

Made some cool changes? - Consider contributing! It is extremely  simple:

- Fork;
- Tweak;
- Pull Request.

> The floor is yours!
