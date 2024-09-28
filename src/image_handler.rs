use image::{ ImageReader, DynamicImage };
use std::io::Cursor;


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

    println!("Image parsed - width: {}, height: {}",
        image.width(),
        image.height()
    );
    image
}
