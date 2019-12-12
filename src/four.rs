pub fn run_a() {
    let mut valid_count = 0;
    for password in 265275..781584 {
        if meets_criteria(&password.to_string(), false) {
            valid_count += 1;
        }
    }
    println!("4a - Valid passwords: {}", valid_count);
}

pub fn run_b() {
    let mut valid_count = 0;
    for password in 265275..781584 {
        if meets_criteria(&password.to_string(), true) {
            valid_count += 1;
        }
    }
    println!("4b - Valid passwords: {}", valid_count);
}

fn meets_criteria(password: &str, part_b: bool) -> bool {
    let digit_list: Vec<_> = password
        .chars()
        .map(|s| s.to_digit(10).expect("Non-digit found"))
        .collect();

    // Check length
    assert_eq!(digit_list.len(), 6);

    let mut repeat_found: bool = false;
    for digit in 1..6 as usize {
        // Check that digits do not decrease from left to right
        if digit_list[digit] < digit_list[digit - 1] {
            return false;
        }

        // Check that there is a repeated digit
        let num_matching_digits = digit_list
            .iter()
            .filter(|&n| *n == digit_list[digit])
            .count();
        if num_matching_digits > 1 && !part_b {
            repeat_found = true;
        } else if num_matching_digits == 2 {
            repeat_found = true;
        }
    }
    repeat_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_for_a() {
        assert_eq!(true, meets_criteria("111111", false));
        assert_eq!(false, meets_criteria("223450", false));
        assert_eq!(false, meets_criteria("123789", false));
    }

    #[test]
    fn tests_for_b() {
        assert_eq!(true, meets_criteria("112233", true));
        assert_eq!(false, meets_criteria("123444", true));
        assert_eq!(true, meets_criteria("111122", true));
    }
}
