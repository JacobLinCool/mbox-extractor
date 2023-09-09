# mbox extractor

Extracts `eml` files from `mbox` file.

## Usage

### Command Line Tool

```sh
❯ mbox-extractor --help
Extract emails from mbox file

Usage: mbox-extractor [OPTIONS] [mbox]

Arguments:
  [mbox]  mbox file to extract [default: -]

Options:
  -o, --output <output>  output directory [default: ./output]
  -h, --help             Print help
  -V, --version          Print version
```

### Library

See [examples/stdio.rs](examples/stdio.rs).
