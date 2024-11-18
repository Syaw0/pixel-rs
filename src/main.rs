use core::str;
use std::{ fs::File, io::{ Error, Read } };

fn is_png_signature(data: &[u8]) -> bool {
    data == b"\x89\x50\x4E\x47\r\n\x1A\n"
}

pub fn crc32(data: &[u8]) -> u32 {
    const POLYNOMIAL: u32 = 0xedb88320;
    let mut crc = 0xffffffff;

    for &byte in data {
        crc = crc ^ (byte as u32);
        for _ in 0..8 {
            if (crc & 1) == 1 {
                crc = (crc >> 1) ^ POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
    }

    !crc
}

fn main() -> Result<(), Error> {
    let file_path = "./src/file.png";
    let mut file = File::open(file_path)?;
    let mut signature = [0; 8];
    file.read_exact(&mut signature)?;
    println!("{:?}", &signature);

    if is_png_signature(&signature) {
        println!("The file is a PNG.");
    } else {
        println!("The file is not a PNG.");
    }

    let mut legnth_bytes = [0; 4];
    file.read_exact(&mut legnth_bytes)?;

    let length = u32::from_be_bytes(legnth_bytes);
    println!("length {length}");

    let mut chunk_type_bytes = [0; 4];
    file.read_exact(&mut chunk_type_bytes)?;
    // let chunk_type = u32::from_be_bytes(chunk_type_bytes);
    let chunk_type_string = str::from_utf8(&chunk_type_bytes).unwrap();

    println!("chunk_type {chunk_type_bytes:?} | str: {chunk_type_string:?}");

    let mut image_header_data_bytes = [0; 13];
    file.read_exact(&mut image_header_data_bytes)?;
    println!("image header {image_header_data_bytes:?}");

    let mut crc_image_header = [0; 4];
    file.read_exact(&mut crc_image_header)?;
    println!("crc value {crc_image_header:?}");

    let mut chunk_data = Vec::new();
    chunk_data.extend_from_slice(&chunk_type_bytes);
    // chunk_data.extend_from_slice(&legnth_bytes);
    chunk_data.extend_from_slice(&image_header_data_bytes);

    // let mut crc_image_header_calculated = 0xffffffff;

    let crc_image_header_calculated_be = crc32(&chunk_data);

    println!("crc value calculated {crc_image_header_calculated_be:?}");
    println!("crc header value :{}", u32::from_be_bytes(crc_image_header));

    if crc_image_header_calculated_be == u32::from_be_bytes(crc_image_header) {
        println!("CRC is okay!");
    } else {
        println!("CRC is not okay!");
    }

    Ok(())
}
