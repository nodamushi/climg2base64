# climg2base64

Encodes an image from the clipboard image or clipboard file path to Base64.

```sh
climg2base64 <FORMAT> [OPTION]
```

- `<FORMAT>` : The desired output image format. `webp`, `png`, `gif`, `bmp`, `jpg`, `tiff`
- `-w` : Maximum output image width[px]. If the clipboard image width exceeds this value, it will be resized.
- `-h` : Max output image height[px].  If the clipboard image height exceeds this value, it will be resized.
- `--ignore-format`: Ignores the `<FORMAT>` argument when the clipboard data is a clipboard file path.
- `--stderr-path`: Outputs the clipboard file path to `stderr` when the clipboard data is a clipboard file path.


## How to build

```sh
cargo build --release --features "file"
```

or

```sh
cargo install --git https://github.com/nodamushi/climg2base64 --features "file"
```

Note: Please remove the features option on Ubuntu 20.04.

### Ubuntu Required Package

```sh
sudo apt install libgtk-3-dev libglib2.0-dev
```

### Help me 😭

I want to build this project for multiple platforms and provide the binaries, but as I'm a beginner with Rust, I'm not sure how to do it.

I would be very grateful if someone could help me.

## License

This project is licensed under either the MIT License or the Unlicense, at your option.


