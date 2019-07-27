# swirlr

Experimental command line app written in [Rust](https://www.rust-lang.org/) which takes an input image and renders an SVG by sampling points along the path of an [Archimedean spiral](https://en.wikipedia.org/wiki/Archimedean_spiral).

The input image will be center-cropped to a square. For best results use a high-contrast input image.

## Building

Build with cargo

```
cargo build --release
```

## Usage

```
swirlr input.jpg > output.svg
```

You may optionally set a color with `--color`.

```
swirlr --color "red" input.jpg > output.svg
```

Note if you're running via cargo don't forget the `--` so cargo doesn't interpret the option on itself.

```
cargo run -- --option "red" input.jpg > output.svg
```

<img src="examples/scream.png?raw=true" width="256" height="256" />
