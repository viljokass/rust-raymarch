use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png;

// Create an image file and save something onto it.
fn main() {
    // Set the path, file and buffer writer
    let path = Path::new(r"./img.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    // Set up the image encoder from png crate
    // with buffer writer from above and image 
    // dimensions of 2 by 2
    let mut encoder = png::Encoder::new(w, 2, 2);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    // Set the writer as encoder's writer
    let mut writer = encoder.write_header().unwrap();

    // Image data
    let data = [255, 0, 0, 255, 255, 255, 0, 255, 0, 0, 0, 255, 0, 255, 0, 255];

    // Write the data
    writer.write_image_data(&data).unwrap();
}
