# swirlr

Experimental command line app written in [Rust](https://www.rust-lang.org/) which takes an input image and renders an SVG by sampling points along the path of an [Archimedean spiral](https://en.wikipedia.org/wiki/Archimedean_spiral).

The input image will be center-cropped to a square. For best results use a high-contrast input image.

## Usage

```
cargo run input.jpg > output.svg
```

<img src="examples/scream.png?raw=true" width="384" height="384" />

To see a version of this compiled to Web Assembly check out [swirlr-wasm](https://github.com/willdady/swirlr-wasm)!
