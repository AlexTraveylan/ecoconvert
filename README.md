# Ecoconvert 🌱

A high-performance command-line tool for converting and optimizing images to modern, bandwidth-efficient formats (WebP, AVIF).

## Features ✨

- Convert images to AVIF and WebP formats
- Resize images (percentage or specific dimensions)
- Quality optimization
- Parallel processing support
- Simple command-line interface

## Installation 🚀

### Prerequisites

- Rust 1.56 or higher
- Cargo (Rust package manager)

### Building from source

```bash
# Clone the repository
git clone https://github.com/AlexTraveylan/ecoconvert.git
cd ecoconvert

# Build with default features (includes parallel processing)
cargo build --release

# The binary will be available at target/release/ecoconvert
```

## Usage

```bash
# Basic conversion
ecoconvert -i input.jpg -o output.avif

# Set quality (0-100)
ecoconvert -i input.jpg -o output.webp --quality 30

# Resize by percentage
ecoconvert -i input.jpg -o output.avif --resize 50%

# Set specific width (maintains aspect ratio)
ecoconvert -i input.jpg -o output.webp --width 800

# Set specific height (maintains aspect ratio)
ecoconvert -i input.jpg -o output.avif --height 600

# Set both width and height
ecoconvert -i input.jpg -o output.webp --width 800 --height 600
```

### Command Options

```
Options:
  -q, --quality <QUALITY>    Set compression quality (0-100) [default: 75]
  -r, --resize <PERCENT>     Resize image by percentage (e.g., "50%")
  -w, --width <WIDTH>        Set specific width in pixels
  -h, --height <HEIGHT>      Set specific height in pixels
  -h, --help                 Print help
  -V, --version             Print version
```

### Supported Formats

Input:
- JPEG/JPG

Output:
- AVIF
- WebP

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.