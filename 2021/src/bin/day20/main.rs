fn parse_input() -> (Vec<u8>, Vec<Vec<u8>>) {
    let (enhancer, image_raw) = include_str!("input.txt").split_once("\n\n").unwrap();
    let input_image = image_raw.lines().map(|l| l.bytes().collect()).collect();
    (enhancer.bytes().collect(), input_image)
}

fn output_pixel(
    input_image: &Vec<Vec<u8>>,
    enhancer: &Vec<u8>,
    y: i32,
    x: i32,
    lit_by_default: bool,
) -> u8 {
    let default_pixel = if lit_by_default { b'#' } else { b'.' };

    let binary_str = [
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
    .into_iter()
    .map(|(iy, ix)| {
        return input_image
            .get(iy as usize)
            .and_then(|row| row.get(ix as usize))
            .unwrap_or(&default_pixel);
    })
    .map(|&c| if c == b'#' { '1' } else { '0' })
    .collect::<String>();

    let index = usize::from_str_radix(&binary_str, 2).unwrap();

    enhancer[index]
}

fn enhance(input_image: &Vec<Vec<u8>>, enhancer: &Vec<u8>, lit_by_default: bool) -> Vec<Vec<u8>> {
    // we pad to account for the image growing by 2 in x and y direction per iteration
    // technically image has an infinite size
    // but only +2 in x and y direction is significant in step
    let mut output_image = vec![vec![b'.'; input_image[0].len() + 2]; input_image.len() + 2];
    for y in 0..output_image.len() {
        for x in 0..output_image[0].len() {
            output_image[y][x] = output_pixel(
                &input_image,
                &enhancer,
                // account for padding by normalizing x andy index
                (y as i32) - 1,
                (x as i32) - 1,
                lit_by_default,
            );
        }
    }

    output_image
}

fn main() {
    let (enhancer, mut image) = parse_input();
    // the input may "blink", meaning:
    // on even iterations the infinite image is filled with # by default and
    // on odd iterations the infinite image is filled with . by default
    let will_blink = if enhancer[0] == b'#' && *enhancer.last().unwrap() == b'.' {
        true
    } else {
        false
    };

    for i in 1..=50 {
        image = enhance(&image, &enhancer, will_blink && i % 2 == 0);

        if i == 2 || i == 50 {
            println!("{}", image.iter().flatten().filter(|c| **c == b'#').count())
        }
    }
}
