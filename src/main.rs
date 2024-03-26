use png;
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;

const DEFAULT_DIM: u32 = 100;

// An enum for defining what to print in the ask_dimension function.
enum Dimension {
    Width,
    Height,
}

// A quick func that asks dimensions.
// input : dimension enum
// output: u32
fn ask_dimension(dim: Dimension) -> u32 {
    // Initialize a mutable string for writing onto
    let mut size = String::new();

    // Depending on the input, print a request
    match dim {
        Dimension::Width => println!("Please input width:"),
        Dimension::Height => println!("Please input height:"),
    }

    // Read line from the standard input
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");

    // Match the parsed Result and return either:
    // * the parsed number, or
    // * the default dimension
    match size.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Unable to parse! Defaulting to {}.", DEFAULT_DIM);
            DEFAULT_DIM
        }
    }
}

// Placeholder for actual drawing
fn draw_color(width: u32, height: u32, data: &mut Vec<u8>) {

    let delta_r: f64 = 1./(width as f64 - 1.);
    let delta_g: f64 = 1./(height as f64 - 1.);

    for i in 0..height {
        let g: u8 = 255 - (255. * delta_g * (i as f64)) as u8;
        for j in 0..width {
            let r: u8 = (255. * delta_r * (j as f64)) as u8;
            data.push(r);
            data.push(g);
            data.push(0);
            data.push(255);    
        }
    }
}


// Create an image file and save something onto it.
fn main() {
    // Ask the image dimensions from the user
    let width = ask_dimension(Dimension::Width);
    let height = ask_dimension(Dimension::Height);

    // For debugging purposes
    println!("Dimensions of the image is {}x{}.", width, height);

    // Set the path, file and buffer writer
    let path = Path::new(r"./img.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    // Set up the image encoder from png crate
    // Currently uses 2 by 2 images.
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    // Set the writer as encoder's writer
    let mut writer = encoder.write_header().expect("Unable to set the writer");

    // Set the data: This would be a good point to split
    let mut data: Vec<u8> = Vec::new();
    draw_color(width, height, &mut data);

    // Write the data
    writer.write_image_data(&data).expect("Unable to write the image data");
}
