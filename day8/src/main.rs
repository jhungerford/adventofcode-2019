use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

/// Parses the given image data into a series of layers.
fn parse_image(width: usize, height: usize, data: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut layers = Vec::new();
    let mut start = 0;
    while start < data.len() {
        let end = start + (width * height) as usize;
        layers.push(data[start..end].to_vec());
        start = end;
    }

    layers.to_owned()
}

/// Returns a checksum for the image, which is the number of 1 digits * the number of 2 digits
/// on the layer with the fewest 0 digits.
fn checksum(layers: &Vec<Vec<u32>>) -> usize {
    let checksum_layer = layers.iter()
        .min_by(|layer1, layer2| num_digits(layer1, 0).cmp(&num_digits(layer2, 0)))
        .unwrap();

    num_digits(checksum_layer, 1) * num_digits(checksum_layer, 2)
}

/// Returns the number of times the given digit shows up in the layer.
fn num_digits(layer: &Vec<u32>, digit: u32) -> usize {
    layer.to_vec().into_iter().filter(|pixel| *pixel == digit).count()
}


/// Given the layers, resolves the image into a viewable picture.  0 represents black pixels,
/// 1 is white, and 2 is transparent.  Higher layers are in front of lower layers, and the first
/// non-transparent pixel is the one shown.
fn resolve_image(layers: &Vec<Vec<u32>>) -> Vec<Pixel> {
    assert!(!layers.is_empty(), "Need at least one layer to resolve the image.");

    let num_pixels = layers[0].len();
    let mut resolved = Vec::with_capacity(num_pixels);

    for i in 0..num_pixels {
        let pixel = layers.iter()
            .map(|layer| layer[i])
            .map(|num| match num {
                0 => Pixel::Black,
                1 => Pixel::White,
                2 => Pixel::Transparent,
                n => panic!("{} is not a valid pixel", n)
            })
            .find(|&pixel| pixel != Pixel::Transparent)
            .unwrap();

        resolved.push(pixel);
    }

    resolved.to_owned()
}

/// Prints out the given image.
fn render_image(width: usize, _height: usize, pixels: &Vec<Pixel>) {
    fn render_pixel(pixel: &Pixel) -> &str {
        match pixel {
            Pixel::Black => " ",
            Pixel::White => "*",
            Pixel::Transparent => " ",
        }
    }

    let mut start = 0;
    while start < pixels.len() {
        let end = start + width;
        let row_str = &mut String::new();
        let row = pixels[start..end].iter()
            .fold(row_str, |s, pixel| { s.push_str(render_pixel(pixel)); s });
        println!("{}", row);
        start = end;
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Pixel {
    Black,
    White,
    Transparent,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_image() {
        let data = vec![1,2,3,4,5,6,7,8,9,0,1,2];
        let parsed = parse_image(3, 2, &data);

        assert_eq!(parsed, vec![
            &[1,2,3,4,5,6],
            &[7,8,9,0,1,2],
        ]);
    }

    #[test]
    fn test_checksum() {
        let data = vec![1,2,3,4,5,6,7,8,9,0,1,2];
        let layers = parse_image(3, 2, &data);

        assert_eq!(checksum(&layers), 1);
    }

    #[test]
    fn test_resolve_image() {
        let data = vec![0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0];
        let layers = parse_image(2, 2, &data);
        let pixels = resolve_image(&layers);

        assert_eq!(pixels, vec![Pixel::Black, Pixel::White, Pixel::White, Pixel::Black]);
        render_image(2, 2, &pixels);
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim().to_string();

    let nums = line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();

    // Part 1: on the line with the fewest 0 digits, what is the number of 1 digits * the number of 2 digits?
    let (width, height) = (25, 6);
    let layers = parse_image(width, height, &nums);

    println!("Part 1: {}", checksum(&layers));

    // Part 2: What message is produced?
    let pixels = resolve_image(&layers);
    render_image(width, height, &pixels);

    Ok(())
}
