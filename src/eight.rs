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
}
