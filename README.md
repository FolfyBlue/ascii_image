# ASCII Art Image Converter

This is a command-line tool written in Rust that converts images into ASCII art. It uses the `clap`, `image`, and `rusttype` libraries to process images and render ASCII characters based on the brightness of the corresponding image pixels.

## Table of Contents

- Usage
- Dependencies
- Installation
- Examples
- License

## Usage

To use this tool, you can run it from the command line with the following options:

```sh
ascii_image --image <image_path> --font <font_path> [--width <output_width>]
```

## Options

- --image or -i: Path to the input image.
- --font or -f: Path to the font file (TTF format) for rendering ASCII characters.
- --width or -w: (Optional) Output image width (in pixels). If not specified, the aspect ratio of the input image is preserved.

## Dependencies

This tool relies on the following Rust crates:

- `clap`: A command-line argument parser.
- `image`: A crate for image processing.
- `rusttype`: A crate for rendering TrueType fonts.

You can find these dependencies in the Cargo.toml file.

## Installation

To build and run the tool, make sure you have Rust installed on your system. Then follow these steps:

1. Clone this repository:

```sh
git clone <https://github.com/your-username/ascii-art-converter.git>
cd ascii-art-converter
```

2. Build the project:

```sh
cargo build --release
```

3. Run the tool:

```sh
cargo run --release -- --image <image_path> --font <font_path> [--width <output_width>]
```

## Examples

### Example 1: Convert an Image

Convert an image to ASCII art using the default font size and preserving the aspect ratio of the input image.

```sh
ascii_image --image example.png --font fonts/IBMPlexMono-Regular.ttf
```

### Example 2: Specify Output Width

Convert an image to ASCII art with a specific output width (e.g., 40 characters wide).

```sh
ascii_image --image example.png --font fonts/IBMPlexMono-Regular.ttf --width 40
```

## License

This project is licensed under the GPLv3 License - see the [LICENSE](LICENSE) file for details.
