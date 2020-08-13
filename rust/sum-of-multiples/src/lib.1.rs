pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..limit {
        if factors.iter().any(|factor| check_factor(i, *factor)) {
            sum += i
        }
    }
    sum
}

fn check_factor(num: u32, factor: u32) -> bool {
    if factor == 0 {
        num == 0
    } else {
        num % factor == 0
    }
}
