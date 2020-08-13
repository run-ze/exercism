pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  (0..limit)
    .filter(|n| factors.iter().any(|factor| check_factor(*n, *factor)))
    .sum()
}

fn check_factor(num: u32, factor: u32) -> bool {
  if factor == 0 {
    num == 0
  } else {
    num % factor == 0
  }
}
