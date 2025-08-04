// Elliott R. Lewandowski
// 2025-08-04
// Flip the least signifigant bit in every channel of every
// pixel in an image

use image::{ImageBuffer, ImageReader, Rgb};
use std::env;

fn main() {
    xor_img();
}

/// xor the least signifigant bit of every channel in every pixel of an image
fn xor_img() {

    //////////////////
    // Verify Usage //
    //////////////////

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("USAGE: {:?} /path/to/img", args[0]);
        return;
    }

    //////////////////////////
    // Load Original Image  //
    //////////////////////////
    
    let img: ImageBuffer<Rgb<u16>, Vec<u16>> = ImageReader::open(args[1].clone())
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb16();

    ///////////////////////
    //  Write New Image  //
    ///////////////////////

    let mut new_img: ImageBuffer<Rgb<u16>, Vec<u16>> = ImageBuffer::new(img.width(), img.height());

    for x in 0..new_img.width() {
        for y in 0..new_img.height() {
            let pixel = img.get_pixel(x, y);
            // Write the old image to the new image with 
            // a flipped LSB in every channel
            new_img.put_pixel(
                x,
                y,
                [
                    pixel.0[0] ^ 0x0001,
                    pixel.0[1] ^ 0x0001,
                    pixel.0[2] ^ 0x0001,
                ]
                .into(),
            );
        }
    }

    ///////////////////////////////
    //  Print Old vs New Pixels  //
    ///////////////////////////////

    for x in 0..5 {
        let old_pixel = img.get_pixel(x, 1);
        let new_pixel = new_img.get_pixel(x, 1);
        println!("Pixel {x}");
        println!("Old Pixel: {:X} {:X} {:X}", old_pixel.0[0], old_pixel.0[1], old_pixel.0[2]);
        println!("New Pixel: {:X} {:X} {:X}\n", new_pixel.0[0], new_pixel.0[1], new_pixel.0[2]);
    }

    let _ = new_img.save("out.png");
}
