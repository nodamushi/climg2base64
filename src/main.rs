use std::{borrow::Cow, env, io::Cursor, process::exit};

use arboard::Clipboard;
use base64::Engine;
use image::ImageFormat;

fn get_format(name: &str) -> Option<image::ImageFormat> {
    let low = name.to_lowercase();
    let low: &str = &low;
    match low {
        "jpeg" | "jpg" => Some(image::ImageFormat::Jpeg),
        "png" => Some(image::ImageFormat::Png),
        "webp" => Some(image::ImageFormat::WebP),
        "bmp" => Some(image::ImageFormat::Bmp),
        "gif" => Some(image::ImageFormat::Gif),
        "avif" => Some(image::ImageFormat::Avif),
        _ => None,
    }
}

fn print_help() {
    let mut args = env::args();
    let p = args.next().unwrap_or("".to_string());
    println!("Usage: {} [-h|--help] <Image Format Name>", p);
    println!("");
    println!("Format:");
    println!("  jpeg, jpg : JPEG");
    println!("  png       : PNG");
    println!("  gif       : GIF");
    println!("  bmp       : bmp");
    println!("  webp      : WEBP");
    println!("  avif      : AVIF");
}

fn get_arg() -> Option<String> {
    let args = env::args();
    if let Some(arg) = args.skip(1).next() {
        if arg == "-h" || arg == "--help" {
            None
        } else {
            Some(arg)
        }
    } else {
        None
    }
}

fn to_binary(width: u32, height: u32, rgba: &[u8], img_fmt: image::ImageFormat) -> Vec<u8> {
    let (data, color) = if img_fmt == ImageFormat::Jpeg {
        let pixels = (width as usize) * (height as usize);
        let mut rgb = Vec::with_capacity(pixels * 3);

        for x in 0..pixels {
            let a = rgba[x * 4 + 3];
            let (r, g, b) = if a == 0 {
                (255, 255, 255) // white
            } else {
                (rgba[x * 4 + 0], rgba[x * 4 + 1], rgba[x * 4 + 2])
            };
            rgb.push(r);
            rgb.push(g);
            rgb.push(b);
        }
        (Cow::Owned(rgb), image::ExtendedColorType::Rgb8)
    } else {
        (Cow::Borrowed(rgba), image::ExtendedColorType::Rgba8)
    };

    let mut buffer = Cursor::new(Vec::new());

    if let Err(e) =
        image::write_buffer_with_format(&mut buffer, data.as_ref(), width, height, color, img_fmt)
    {
        eprintln!("Fail to create image: {}", e);
        exit(1)
    }
    buffer.into_inner()
}

fn main() {
    let img_fmt_name = match get_arg() {
        Some(x) => x,
        None => {
            print_help();
            exit(1)
        }
    };
    let img_fmt = if let Some(x) = get_format(&img_fmt_name) {
        x
    } else {
        eprintln!("Invalid format name '{}'", img_fmt_name);
        print_help();
        exit(1)
    };

    let binary = {
        let mut clipboad = match Clipboard::new() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Fail to init clipboard. {}", e);
                exit(1)
            }
        };

        let img = match clipboad.get_image() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Clipboad has no image.");
                exit(1)
            }
        };
        let (rgba, width, height) = (img.bytes.as_ref(), img.width as u32, img.height as u32);

        to_binary(width, height, rgba, img_fmt)
    };

    println!("{}", base64::prelude::BASE64_STANDARD.encode(binary));
}
