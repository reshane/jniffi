use image::{ ImageReader, DynamicImage };
use std::io::Cursor;
use image::imageops::FilterType;


pub fn get_image_from_bytes(bytes: Vec<u8>) -> DynamicImage {
    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format();
    let reader = reader.map_err(|e| {
        eprintln!("Error creating reader: {}", e);
    }).unwrap();
    println!("Image format determined: {:?}",
             reader.format().unwrap()
    );
    let image = reader.decode().map_err(|e| {
        eprintln!("Error decoding data: {}", e);
    }).unwrap();

    image
}

pub fn resize_dynamic_image(
    nwidth: u32, 
    nheight: u32, 
    image: DynamicImage
) -> DynamicImage {
    image.resize(nwidth, nheight, FilterType::Gaussian)
}

pub fn get_png_bytes(img: DynamicImage) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut bytes),
        image::ImageFormat::Png
    ).expect("Could not resize image!");
    bytes
}

