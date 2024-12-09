use core::str;
use std::{ fs::File, io::{ Error, Read }, vec };
use flate2::read::ZlibDecoder;

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

fn split_chunks(mut file: &File) -> Result<String, Error> {
    // CONSTANT
    let width = 1600;
    let height = 900;
    let color_type = 6;
    let bit_per_pixel = 8;

    let mut legnth_bytes = [0; 4];
    file.read_exact(&mut legnth_bytes)?;

    let length = u32::from_be_bytes(legnth_bytes) as usize;
    println!("length {length}");

    let mut chunk_type_bytes = [0; 4];
    file.read_exact(&mut chunk_type_bytes)?;
    // let chunk_type = u32::from_be_bytes(chunk_type_bytes);
    let chunk_type_string = str::from_utf8(&chunk_type_bytes).unwrap();

    println!("chunk_type {chunk_type_bytes:?} | str: {chunk_type_string:?}");

    let mut image_data_bytes = vec![0; length];
    file.read_exact(&mut image_data_bytes)?;
    println!("image data {:?}", image_data_bytes.len());

    if chunk_type_string == "IHDR" {
        println!("{image_data_bytes:?}");
    }

    if chunk_type_string == "IDAT" {
        let mut decoder = ZlibDecoder::new(&image_data_bytes[..]);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data).unwrap();
        let buffer = decompressed_data;
        println!("{} {}", buffer.len(), image_data_bytes.len());

        let filter_type_byte = &buffer[0];
        let filter_type_int = u8::from_be_bytes([filter_type_byte.clone()]);
        println!("Filter type : {filter_type_int}");

        let mut image_data = Vec::new();
        image_data.extend_from_slice(&buffer[1..]); // slice the first byte
        // because filter type is always is (0) in PNG

        // let mut pixels = Vec::new();
    }

    let mut image_crc = [0; 4];
    file.read_exact(&mut image_crc)?;
    println!("crc value {image_crc:?}");
    let mut chunk_data = Vec::new();
    chunk_data.extend_from_slice(&chunk_type_bytes);
    // chunk_data.extend_from_slice(&legnth_bytes);
    chunk_data.extend_from_slice(&image_data_bytes);

    // let mut crc_image_header_calculated = 0xffffffff;

    let crc_image_header_calculated_be = crc32(&chunk_data);

    // println!("crc value calculated {crc_image_header_calculated_be:?}");
    // println!("crc header value :{}", u32::from_be_bytes(image_crc));

    if crc_image_header_calculated_be == u32::from_be_bytes(image_crc) {
        println!("CRC is okay!");
    } else {
        println!("CRC is not okay!");
    }
    Ok(chunk_type_string.to_string())
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

    let mut is_done = false;
    while !is_done {
        let t = split_chunks(&file)?;
        println!("\n");
        if t == "IEND" {
            is_done = true;
        }
    }

    return Ok(());

    // let mut legnth_bytes = [0; 4];
    // file.read_exact(&mut legnth_bytes)?;

    // let length = u32::from_be_bytes(legnth_bytes);
    // println!("length {length}");

    // let mut chunk_type_bytes = [0; 4];
    // file.read_exact(&mut chunk_type_bytes)?;
    // // let chunk_type = u32::from_be_bytes(chunk_type_bytes);
    // let chunk_type_string = str::from_utf8(&chunk_type_bytes).unwrap();

    // println!("chunk_type {chunk_type_bytes:?} | str: {chunk_type_string:?}");

    // let mut image_header_data_bytes = [0; 13];
    // file.read_exact(&mut image_header_data_bytes)?;
    // println!("image header {image_header_data_bytes:?}");

    // let image_width: [u8; 4] = image_header_data_bytes[0..4].try_into().expect("Error");
    // println!("width:{}px", u32::from_be_bytes(image_width.clone()));

    // let image_height: [u8; 4] = image_header_data_bytes[4..8].try_into().expect("Error");
    // println!("height:{}px", u32::from_be_bytes(image_height.clone()));

    // let image_bit_deps: [u8; 1] = image_header_data_bytes[8..9].try_into().expect("Error");
    // println!(
    //     "image_bit_deps:{} number of bits per pixel",
    //     u8::from_be_bytes(image_bit_deps.clone())
    // );

    // let image_color_type: [u8; 1] = image_header_data_bytes[9..10].try_into().expect("Error");
    // println!(
    //     "image_color_type:{} 0 = grayscale, 2 = RGB, 3 = palette, 4 = grayscale with alpha, 6 = RGBA",
    //     u8::from_be_bytes(image_color_type.clone())
    // );

    // let image_compression_method: [u8; 1] = image_header_data_bytes[10..11]
    //     .try_into()
    //     .expect("Error");
    // println!(
    //     "image_compression_method:{} always 0 = deflate/inflate",
    //     u8::from_be_bytes(image_compression_method.clone())
    // );

    // let image_filter_method: [u8; 1] = image_header_data_bytes[11..12].try_into().expect("Error");
    // println!(
    //     "image_filter_method:{} always 0 = none",
    //     u8::from_be_bytes(image_filter_method.clone())
    // );

    // let image_iterlace_method: [u8; 1] = image_header_data_bytes[12..13].try_into().expect("Error");
    // println!(
    //     "image_iterlace_method:{}  0 = none, 1 = Adam7",
    //     u8::from_be_bytes(image_iterlace_method.clone())
    // );

    // println!(
    //     "{image_width:?} {image_height:?} {image_bit_deps:?} {image_color_type:?} {image_compression_method:?} {image_filter_method:?} {image_iterlace_method:?}"
    // );

    // let mut crc_image_header = [0; 4];
    // file.read_exact(&mut crc_image_header)?;
    // println!("crc value {crc_image_header:?}");
    // let mut chunk_data = Vec::new();
    // chunk_data.extend_from_slice(&chunk_type_bytes);
    // // chunk_data.extend_from_slice(&legnth_bytes);
    // chunk_data.extend_from_slice(&image_header_data_bytes);

    // // let mut crc_image_header_calculated = 0xffffffff;

    // let crc_image_header_calculated_be = crc32(&chunk_data);

    // println!("crc value calculated {crc_image_header_calculated_be:?}");
    // println!("crc header value :{}", u32::from_be_bytes(crc_image_header));

    // if crc_image_header_calculated_be == u32::from_be_bytes(crc_image_header) {
    //     println!("CRC is okay!");
    // } else {
    //     println!("CRC is not okay!");
    // }

    // // =============================

    // let mut legnth_bytes = [0; 4];
    // file.read_exact(&mut legnth_bytes)?;

    // let length = u32::from_be_bytes(legnth_bytes) as usize;
    // println!("length {length}");

    // let mut chunk_type_bytes = [0; 4];
    // file.read_exact(&mut chunk_type_bytes)?;
    // // let chunk_type = u32::from_be_bytes(chunk_type_bytes);
    // let chunk_type_string = str::from_utf8(&chunk_type_bytes).unwrap();

    // println!("chunk_type {chunk_type_bytes:?} | str: {chunk_type_string:?}");

    // let mut image_data_bytes = vec![0; length];
    // file.read_exact(&mut image_data_bytes)?;
    // println!("image data {}", image_data_bytes.len());

    // let mut image_crc: [i32; 4] = [0; 4];
    // file.read_exact(&mut crc_image_header)?;
    // println!("crc value {image_crc:?}");
    // let mut chunk_data = Vec::new();
    // chunk_data.extend_from_slice(&chunk_type_bytes);
    // // chunk_data.extend_from_slice(&legnth_bytes);
    // chunk_data.extend_from_slice(&image_data_bytes);

    // // let mut crc_image_header_calculated = 0xffffffff;

    // let crc_image_header_calculated_be = crc32(&chunk_data);

    // println!("crc value calculated {crc_image_header_calculated_be:?}");
    // println!("crc header value :{}", u32::from_be_bytes(crc_image_header));

    // if crc_image_header_calculated_be == u32::from_be_bytes(crc_image_header) {
    //     println!("CRC is okay!");
    // } else {
    //     println!("CRC is not okay!");
    // }

    // // ============================================================

    // let mut legnth_bytes = [0; 4];
    // file.read_exact(&mut legnth_bytes)?;

    // let length = u32::from_be_bytes(legnth_bytes) as usize;
    // println!("length {length}");

    // let mut chunk_type_bytes = [0; 4];
    // file.read_exact(&mut chunk_type_bytes)?;
    // // let chunk_type = u32::from_be_bytes(chunk_type_bytes);
    // let chunk_type_string = str::from_utf8(&chunk_type_bytes).unwrap();

    // println!("chunk_type {chunk_type_bytes:?} | str: {chunk_type_string:?}");

    // let mut image_data_bytes = vec![0; length];
    // file.read_exact(&mut image_data_bytes)?;
    // println!("image data {}", image_data_bytes.len());

    // let mut image_crc: [i32; 4] = [0; 4];
    // file.read_exact(&mut crc_image_header)?;
    // println!("crc value {image_crc:?}");
    // let mut chunk_data = Vec::new();
    // chunk_data.extend_from_slice(&chunk_type_bytes);
    // // chunk_data.extend_from_slice(&legnth_bytes);
    // chunk_data.extend_from_slice(&image_data_bytes);

    // // let mut crc_image_header_calculated = 0xffffffff;

    // let crc_image_header_calculated_be = crc32(&chunk_data);

    // println!("crc value calculated {crc_image_header_calculated_be:?}");
    // println!("crc header value :{}", u32::from_be_bytes(crc_image_header));

    // if crc_image_header_calculated_be == u32::from_be_bytes(crc_image_header) {
    //     println!("CRC is okay!");
    // } else {
    //     println!("CRC is not okay!");
    // }

    // Ok(())
}
