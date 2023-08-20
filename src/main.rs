use clap::Parser;
use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView};
use rusttype::{Font, Scale};
use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image path
    #[arg(short, long)]
    image: String,

    /// Font path
    #[arg(short, long)]
    font: String,

    /// Image width (Keeps aspect ratio)
    #[arg(short, long)]
    width: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let font = args.font;

    let font_data = fs::read(font).expect("Could not read font file");
    let font = Font::try_from_bytes(&font_data).expect("Failed to load font");

    let font_size = 10.0; // Adjust the font size as needed
    let scale = Scale {
        x: font_size,
        y: font_size,
    };

    // Glyphs to draw for all ascii chars. Feel free to try other strings.
    let mut ascii_chars = String::new();
    // List of printable ascii characters
    for i in 32..126 {
        ascii_chars.push(char::from(i))
    }

    let mut character_brightness: HashMap<char, f32> = HashMap::new();
    for character in ascii_chars.chars() {
        let mut whiteness: f32 = 0.0;
        // Render the character glyph
        let glyph = font.glyph(character).scaled(scale);
        let glyph_position = &glyph.positioned(rusttype::point(0.0, 0.0));

        // Draw the glyph on the image buffer
        glyph_position.draw(|_x_pos, _y_pos, coverage| whiteness += coverage);

        character_brightness.insert(character, whiteness);
    }

    // Sort character_brightness by darkest to lightest
    let mut sorted_pairs: Vec<(&char, &f32)> = character_brightness.iter().collect();
    sorted_pairs.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal));

    let palette: Vec<char> = sorted_pairs.iter().map(|(k, _)| **k).collect();
    println!("Palette: {:?}", palette);

    // Open the input image
    let mut input_image = ImageReader::open(args.image)
        .expect("Could not open image")
        .decode()
        .expect("Could not decode image");
    let (mut image_width, mut image_height) = input_image.dimensions();

    if !args.width.is_none() {
        input_image = input_image.resize(args.width.unwrap(), image_height, FilterType::Lanczos3);
        (image_width, image_height) = input_image.dimensions();
    }

    let char_rect = font.glyph('█').scaled(scale).exact_bounding_box().unwrap();
    let image_width_multiplicator = char_rect.height() / char_rect.width();

    let resized_image = input_image.resize_exact(
        (image_width as f32 * image_width_multiplicator) as u32,
        image_height,
        FilterType::Lanczos3,
    );
    (image_width, _) = resized_image.dimensions();

    // Generate text from image
    let mut output_text = String::new();
    let image_rgb = resized_image.into_rgb8();
    let mut string_width = 0;
    for pixel in image_rgb.pixels() {
        let (r, g, b) = (
            *pixel.0.get(0).unwrap() as f64,
            *pixel.0.get(1).unwrap() as f64,
            *pixel.0.get(2).unwrap() as f64,
        );
        let weight = ((0.2126 * r) + (0.7152 * g) + (0.0722 * b)) / 254.9;

        let pixel_char = palette[((palette.len() - 1) as f64 * weight) as usize];
        output_text.push(pixel_char);
        string_width += 1;
        if string_width == image_width {
            string_width = 0;
            output_text.push('\n');
        }
    }

    fs::write("output.txt", &output_text).expect("Unable to write file");
    println!("output.txt:\n{}", output_text)
}
