fn parse_input() -> (String, Vec<Vec<char>>) {
    let (enhancer, image_raw) = include_str!("input.txt").split_once("\n\n").unwrap();
    let input_image = image_raw.lines().map(|l| l.chars().collect()).collect();
    (enhancer.to_string(), input_image)
}

fn output_pixel(
    input_image: &Vec<Vec<char>>,
    enhancer: &str,
    y: i32,
    x: i32,
    lit_by_default: bool,
) -> char {
    let binary_str = vec![
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ]
    .iter()
    .map(|&(iy, ix)| {
        if let Some(pixel) = input_image
            .get(iy as usize)
            .and_then(|row| row.get(ix as usize))
        {
            return *pixel;
        }

        if lit_by_default {
            return '#';
        } else {
            return '.';
        }
    })
    .map(|c| if c == '#' { '1' } else { '0' })
    .collect::<String>();

    let index = usize::from_str_radix(&binary_str, 2).unwrap();

    enhancer.chars().nth(index).unwrap()
}

fn main() {
    let (enhancer, mut image) = parse_input();
    // The real input may "blink", meaning:
    // on even iterations the infinite image is filled with # by default and
    // on odd iterations the infinite image is filled with . by default
    let will_blink =
        if enhancer.chars().nth(0).unwrap() == '#' && enhancer.chars().last().unwrap() == '.' {
            true
        } else {
            false
        };

    let pad = 2;
    for i in 1..=50 {
        // we pad to account for the image growing by 2 in xy direction per iteration
        // technically image is sized infinitely
        // but only +2 in xy direction is signification across  a set of 2 iterations
        let mut output_image =
            vec![vec!['.'; image[0].len() + pad as usize]; image.len() + pad as usize];
        for y in 0..output_image.len() {
            for x in 0..output_image[0].len() {
                output_image[y][x] = output_pixel(
                    &image,
                    &enhancer,
                    // account for padding by normalizing xy
                    (y as i32) - (pad / 2),
                    (x as i32) - (pad / 2),
                    will_blink && i % 2 == 0,
                );
            }
        }
        image = output_image;

        if i == 2 || i == 50 {
            println!("{}", image.iter().flatten().filter(|c| **c == '#').count())
        }
    }
}
