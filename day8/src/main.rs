use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

/// Parses the given image data into a series of layers.
fn parse_image(width: u32, height: u32, data: &Vec<u32>) -> Vec<&[u32]> {
    let mut layers = Vec::new();
    let mut start = 0;
    while start < data.len() {
        let end = start + (width * height) as usize;
        layers.push(&data[start..end]);
        start = end;
    }

    layers.to_owned()
}

/// Returns a checksum for the image, which is the number of 1 digits * the number of 2 digits
/// on the layer with the fewest 0 digits.
fn checksum(layers: &Vec<&[u32]>) -> usize {
    let checksum_layer = layers.iter()
        .min_by(|layer1, layer2| num_digits(layer1, 0).cmp(&num_digits(layer2, 0)))
        .unwrap();

    num_digits(*checksum_layer, 1) * num_digits(*checksum_layer, 2)
}

fn num_digits(layer: &[u32], digit: u32) -> usize {
    layer.to_vec().into_iter().filter(|pixel| *pixel == digit).count()
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

        assert_eq!(1, checksum(&layers));
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
    let layers = parse_image(25,6, &nums);

    println!("Part 1: {}", checksum(&layers));

    Ok(())
}
