use oxipng::InFile;
use oxipng::OutFile;
use oxipng::Options;
use log::{error, info};
use log4rs;

// lib.rs, simple FFI code
#[no_mangle]
pub unsafe extern "C" fn oxicompress(utf16_str: *const u16, utf16_len: i32)
{
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    match oxipng::optimize(
        &csharp_to_rust_string(utf16_str, utf16_len).into(), 
        &OutFile::Path(None), 
        &Options::default(),
    ) 
    {
        //log ok
        Ok(_) => info!("success"),
        //log not oke
        Err(_) => error!("compress error"),
    }
    

}

pub unsafe fn csharp_to_rust_string(utf16_str: *const u16, utf16_len: i32) -> String {
    let slice = std::slice::from_raw_parts(utf16_str, utf16_len as usize);
    String::from_utf16(slice).unwrap()
}
