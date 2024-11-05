use std::{
    borrow::Cow,
    fs,
    io::{Cursor, Read},
    path::Path,
    process::exit,
};

use arboard::Clipboard;
use base64::Engine;
use clap::Parser;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, ImageFormat, Rgb, Rgba};

/// Clipboard image to base64
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag = true)]
struct Arg {
    /// Output image format. [webp, png, gif, bmp, jpg, tiff]
    format: String,
    /// Max image width (px)
    #[arg(short, long)]
    width: Option<u32>,
    /// Max image height (px)
    #[arg(short, long)]
    height: Option<u32>,

    /// Ignore output image format when the clipboard is a file path.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    ignore_format: bool,

    /// Output the clipboard file path to the stderr.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    stderr_path: bool,

    /// Show help message
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
}

impl Arg {
    fn get_image_width(&self, w: u32, h: u32) -> (u32, u32, f32, bool) {
        match (self.width, self.height) {
            (Some(w2), Some(h2)) => {
                let v1 = (w2 as f32) / (w as f32);
                let v2 = (h2 as f32) / (h as f32);
                if v1 < v2 {
                    if w <= w2 {
                        (w, h, 1.0, false)
                    } else {
                        let h3 = ((h as f32) * v1) as u32;
                        (w2, h3, v1, true)
                    }
                } else {
                    if h <= h2 {
                        (w, h, 1.0, false)
                    } else {
                        let w3 = ((w as f32) * v2) as u32;
                        (w3, h2, v2, true)
                    }
                }
            }
            (Some(w2), None) => {
                if w <= w2 {
                    (w, h, 1.0, false)
                } else {
                    let v1 = (w2 as f32) / (w as f32);
                    let h3 = ((h as f32) * v1) as u32;
                    (w2, h3, v1, true)
                }
            }
            (None, Some(h2)) => {
                if h <= h2 {
                    (w, h, 1.0, false)
                } else {
                    let v2 = (h2 as f32) / (h as f32);
                    let w3 = ((w as f32) * v2) as u32;
                    (w3, h2, v2, true)
                }
            }
            (None, None) => (w, h, 1.0, false),
        }
    }
}

macro_rules! clipboard_empty {
    () => {
        eprintln!("Clipboad has no image.");
        exit(2)
    };
}

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

fn print_formats() {
    println!("Format:");
    println!("  jpeg, jpg : JPEG");
    println!("  png       : PNG");
    println!("  gif       : GIF");
    println!("  bmp       : bmp");
    println!("  webp      : WEBP");
    println!("  avif      : AVIF");
}

fn to_binary(width: u32, height: u32, rgba: &[u8], arg: &Arg) -> Vec<u8> {
    let img_fmt = if let Some(x) = get_format(&arg.format) {
        x
    } else {
        eprintln!("Invalid format name '{}'", arg.format);
        print_formats();
        exit(1)
    };

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

    let (w, h, _, r) = arg.get_image_width(width, height);
    // Resize
    if r {
        type SliceRgb<'a> = ImageBuffer<Rgb<u8>, &'a [u8]>;
        type SliceRgba<'a> = ImageBuffer<Rgba<u8>, &'a [u8]>;
        if color == image::ExtendedColorType::Rgb8 {
            if let Some(img) = SliceRgb::from_raw(width, height, &data) {
                let resized = image::imageops::resize(&img, w, h, FilterType::Gaussian);
                match resized.write_to(&mut buffer, img_fmt) {
                    Ok(()) => return buffer.into_inner(),
                    Err(e) => {
                        eprintln!("Fail to create image binary: {}", e);
                        exit(2)
                    }
                }
            } // (ignore error)?
        } else {
            if let Some(img) = SliceRgba::from_raw(width, height, &data) {
                let resized = image::imageops::resize(&img, w, h, FilterType::Gaussian);
                match resized.write_to(&mut buffer, img_fmt) {
                    Ok(()) => return buffer.into_inner(),
                    Err(e) => {
                        eprintln!("Fail to create image binary: {}", e);
                        exit(2)
                    }
                }
            } // (ignore error)?
        }
    }

    if let Err(e) =
        image::write_buffer_with_format(&mut buffer, data.as_ref(), width, height, color, img_fmt)
    {
        eprintln!("Fail to create image: {}", e);
        exit(2)
    }
    buffer.into_inner()
}

fn dynimg_to_vec(image: DynamicImage, save_fmt: ImageFormat) -> Vec<u8> {
    let mut c = Cursor::new(Vec::new());
    if save_fmt == ImageFormat::Jpeg {
        if let None = image.as_rgb8() {
            let rgb8 = image.to_rgb8();
            match rgb8.write_to(&mut c, save_fmt) {
                Ok(()) => return c.into_inner(),
                Err(e) => {
                    eprintln!("Fail to create image binary: {}", e);
                    exit(2)
                }
            }
        }
    }
    match image.write_to(&mut c, save_fmt) {
        Ok(()) => return c.into_inner(),
        Err(e) => {
            eprintln!("Fail to create image binary: {}", e);
            exit(2)
        }
    }
}

fn read_image_file(p: &Path, arg: &Arg) -> Vec<u8> {
    let buf = if let Ok(mut file) = fs::File::open(p) {
        let mut buf = Vec::new();
        if let Err(_) = file.read_to_end(&mut buf) {
            clipboard_empty!();
        }
        buf
    } else {
        clipboard_empty!();
    };

    let img_fmt = if let Ok(x) = image::guess_format(&buf) {
        x
    } else {
        clipboard_empty!();
    };
    let save_fmt = if arg.ignore_format {
        img_fmt
    } else {
        get_format(&arg.format).unwrap_or(img_fmt)
    };

    if let Ok(img) = image::load_from_memory(&buf) {
        let (w, h, _, r) = arg.get_image_width(img.width(), img.height());
        if r {
            let resized = img.resize(w, h, FilterType::Gaussian);
            return dynimg_to_vec(resized, save_fmt);
        }
        if img_fmt != save_fmt {
            return dynimg_to_vec(img, save_fmt);
        }
    } else {
        clipboard_empty!();
    }
    buf
}

fn main() {
    let arg = Arg::parse();

    let (binary, path) = {
        let mut clipboad = match Clipboard::new() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Fail to init clipboard. {}", e);
                exit(2)
            }
        };

        if let Ok(img) = clipboad.get_image() {
            let (rgba, width, height) = (img.bytes.as_ref(), img.width as u32, img.height as u32);
            (to_binary(width, height, rgba, &arg), None)
        } else if let Ok(files) = clipboard_files::read() {
            if !files.is_empty() {
                (read_image_file(&files[0], &arg), Some(files[0].clone()))
            } else {
                clipboard_empty!();
            }
        } else {
            clipboard_empty!();
        }
    };

    println!("{}", base64::prelude::BASE64_STANDARD.encode(binary));
    if let Some(path) = path {
        if arg.stderr_path {
            eprintln!("{}", path.display());
        }
    }
}
