use std::fs::File;

use image::{codecs::png::PngEncoder, ColorType, GenericImageView, ImageEncoder};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    println!("Hello from thumbhash");

    let img = image::open("input.jpg")?;
    let (width, height) = img.dimensions();
    dbg!((width, height));

    let mut rgba_values = Vec::with_capacity((width * height * 4) as usize);

    for (_, _, pixel) in img.pixels() {
        rgba_values.extend_from_slice(&pixel.0[..]);
    }

    let hash = thumbhash::rgba_to_thumb_hash(width as _, height as _, &rgba_values[..]);
    println!("{hash:x?}");

    let (width, height, pixels) = thumbhash::thumb_hash_to_rgba(&hash[..]).unwrap();

    let output_path = "output.png";
    let enc = PngEncoder::new(File::create(output_path)?);
    enc.write_image(&pixels[..], width as _, height as _, ColorType::Rgba8)?;
    println!("Wrote {}", output_path);

    Ok(())
}
