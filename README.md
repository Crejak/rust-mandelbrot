#Mandelbrot

Mandelbrot fractal generator and viewer. Uses the SFML (sfml-dev.org) and rust-sfml (rust-sfml.org) to generate and display the image.

##Installation

Works on Linux 64bits (only tested on Debian Jessie - amd64), but it should work on most OS.

###Prerequisites

Make sure you've installed SFML and CSFML 2.0 (available on sfml-dev.org).
You should also install the latest rustc and Cargo version (rust-lang.org).
Then install the rust-sfml crate (you can download it from Github, as it isn't available on crates.io for the moment).

###Download and install

1. Download the git package (either with `git clone` or in zip format).
2. Extract here (if it's a zip) and go in the directory (the one called _rust-mandelbrot_).
3. Type `cargo build` to build it, and `cargo build --release` to have some optimizations.
4. Launch it with `cargo run`.

##Use

The main view is the Mandelbrot set in a 2D graph (upper left corner is at -2; 1 and lower right 1; 1). You can zoom in with the mouse (a rectangle shape show you where you want to zoom in), just hit the left button to do so. If you want to get the original view, just hit the right button.
