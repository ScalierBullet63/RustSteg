# RustSteg

CLI steganography tool written in Rust. Hide encrypted messages inside images.

## Build

```bash
cargo build --release
```

## Usage

```bash
# Encode a message into an image
ruststeg encode -t image.png -m "secret message" [--encrypt]

# Decode a message from an image
ruststeg decode -t image.png
```
