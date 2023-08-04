use image::io::Reader;
use image::ImageFormat;
use imagequant::RGBA;
use oxipng::Options;
use image::GenericImageView;
use oxipng::OutFile;
use std::env;
use std::fs::File;
use std::io::{self, Result};
use std::path::{Path, PathBuf};

use lz4::{Decoder, EncoderBuilder};

// lib.rs, simple FFI code
#[no_mangle]
pub unsafe extern "C" fn oxicompress(utf16_str: *const u16, utf16_len: i32) -> i32 {
    match oxipng::optimize(
        &csharp_to_rust_string(utf16_str, utf16_len).into(),
        &OutFile::Path(None),
        &Options::default(),
    ) {
        //log ok
        Ok(_) => 1,
        //log not oke
        Err(_) => 0
    }
}



fn compress(source: &Path, destination: &Path) -> Result<()> {
    println!("Compressing: {} -> {}", source.display(), destination.display());

    let mut input_file = File::open(source)?;
    let output_file = File::create(destination)?;
    let mut encoder = EncoderBuilder::new()
        .level(4)
        .build(output_file)?;
    io::copy(&mut input_file, &mut encoder)?;
    let (_output, result) = encoder.finish();
    result
}

fn decompress(source: &Path, destination: &Path) -> Result<()> {
    println!("Decompressing: {} -> {}", source.display(), destination.display());

    let input_file = File::open(source)?;
    let mut decoder = Decoder::new(input_file)?;
    let mut output_file = File::create(destination)?;
    io::copy(&mut decoder, &mut output_file)?;

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn lz4compress(utf16_str: *const u16, utf16_len: i32) {
    compress(&Path::new(&csharp_to_rust_string(utf16_str, utf16_len)), &Path::new(&csharp_to_rust_string(utf16_str, utf16_len)).with_extension("lz4")).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn lz4decompress(utf16_str: *const u16, utf16_len: i32) {
    decompress(&Path::new(&csharp_to_rust_string(utf16_str, utf16_len)), &Path::new(&csharp_to_rust_string(utf16_str, utf16_len)).with_extension("")).unwrap();
}





#[no_mangle]
pub unsafe extern "C" fn quantcompress(utf16_str: *const u16, utf16_len: i32) {
    // Configure the library
    let mut liq = imagequant::new();
    liq.set_speed(10).unwrap();

    let imageSrc =
        (image::io::Reader::open(&Path::new(&csharp_to_rust_string(utf16_str, utf16_len)))).unwrap().decode().unwrap();

    let img = imagequant::Image::new(
        &liq,
        imageSrc.into_rgba32f().into_raw().iter().map(|&x| x.to_rgba()).collect(),
        imageSrc.width(),
        imageSrc.height(),
        0,
    ).unwrap();

    match liq.quantize(&mut img) {
        Ok(res) => {
            //save image to imgSrc
            imageSrc.save(&csharp_to_rust_string(utf16_str, utf16_len).into());
            {}
        }
        Err(_) => {}
    };
}
 
// lib.rs, simple FFI code
#[no_mangle]
pub extern "C" fn test() -> i32 {
    10
}

pub unsafe fn csharp_to_rust_string(utf16_str: *const u16, utf16_len: i32) -> String {
    let slice = std::slice::from_raw_parts(utf16_str, utf16_len as usize);
    String::from_utf16(slice).unwrap()
}
