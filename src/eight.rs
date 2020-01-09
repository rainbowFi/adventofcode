pub fn run_a() {
    let image_data = std::fs::read_to_string("inputs/input_8.txt").expect("Unable to read file");
    let layers = transform_image_into_layers(&image_data, 25, 6);
    let mut fewest_zeros: (usize, usize) = (std::usize::MAX, 0);
    for (index, layer) in layers.iter().enumerate() {
        let num_zeros = layer.iter().filter(|&n| *n == 0).count();
        if num_zeros < fewest_zeros.0 {
            fewest_zeros.0 = num_zeros;
            fewest_zeros.1 = index;
        }
    }

    let num_1s = layers[fewest_zeros.1].iter().filter(|&n| *n == 1).count();
    let num_2s = layers[fewest_zeros.1].iter().filter(|&n| *n == 2).count();
    let result = num_1s * num_2s;
    println!("8a: result {}", result);
}

pub fn run_b() {
    let image_data = std::fs::read_to_string("inputs/input_8.txt").expect("Unable to read file");
    let mut decoded_image = decode_image(&image_data, 25, 6);
    //println!("{}", decoded_image);
    println!("8b: ");
    for _ in 0..6 {
        let remaining = decoded_image.split_off(25);
        println!("{}", decoded_image);
        decoded_image = remaining;
    }
}

fn transform_image_into_layers(image_data: &String, width: usize, height: usize) -> Vec<Vec<i32>> {
    let image_as_nums: Vec<i32> = image_data
        .trim()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as i32)
        .collect();
    let result = image_as_nums
        .chunks(width * height)
        .map(|layer| layer.to_vec())
        .collect();
    //println!("Result: {:?}", result);
    result
}

fn decode_image(image_data: &String, width: usize, height: usize) -> String {
    let layers = transform_image_into_layers(image_data, width, height);
    determine_pixel_states(layers)
}

fn determine_pixel_states(layers: Vec<Vec<i32>>) -> String {
    let mut pixel_states = vec![];
    let mut first_layer = true;

    for layer in layers {
        // this is returning a borrow of the i32 as pixel, not the value?
        for (index, pixel) in layer.iter().enumerate() {
            if first_layer {
                // first layer is all visible
                pixel_states.push(*pixel);
            } else {
                // later layers only visible if previous layer is transparent
                if pixel_states[index] == 2 {
                    pixel_states[index] = *pixel;
                }
            }
        }

        first_layer = false;
    }

    let pixel_chars: Vec<String> = pixel_states
        .iter()
        .map(|pixel| (if *pixel == 0 { " " } else { "#" }).to_string())
        .collect();
    pixel_chars.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_for_a() {
        assert_eq!(
            transform_image_into_layers(&String::from("123456789012"), 3, 2),
            vec![[1, 2, 3, 4, 5, 6], [7, 8, 9, 0, 1, 2]]
        );
    }

    #[test]
    fn tests_for_b() {
        assert_eq!(
            decode_image(&String::from("0222112222120000"), 2, 2),
            " ## "
        );
    }
}
