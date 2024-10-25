use std::{fs, io::Write};

pub fn save_to_tga(width: i32, height: i32, data: Vec<u8>) {
    let mut output: Vec<u8> = vec![];

    // https://en.wikipedia.org/wiki/Truevision_TGA#Header

    output.push(0); // Does not contain image ID field
    output.push(0); // Does not contain color map
    output.push(3); // Uncompressed grayscale image format
    for _ in 0..7 {
        output.push(0); // Null colormap, and x origin
    }

    let mut y_origin = (height as u16).to_be_bytes();
    y_origin.reverse();
    for byte in y_origin {
        output.push(byte);
    }

    let mut image_width = (width as u16).to_be_bytes(); // Image width
    image_width.reverse();
    for byte in image_width {
        output.push(byte);
    }

    for byte in y_origin { // Image height
        output.push(byte);
    }

    output.push(8); // Bits per pixel
    output.push(0b00001000); // Image descriptor

    output.extend(data);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("output.tga").unwrap();

    file.write_all(&output).unwrap();
}