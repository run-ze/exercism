pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let num_string_len = num_string.len() as u32;
    num == num_string
        .chars()
        .map(|s| s.to_digit(10).unwrap().pow(num_string_len))
        .sum()
}
