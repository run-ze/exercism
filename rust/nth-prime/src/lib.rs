pub fn nth(n: u32) -> u32 {
    let mut index = 0;
    for i in 2.. {
        if is_prime(i) {
            if index == n {
                return i;
            } else {
                index += 1;
            }
        }
    }
    return 0;
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    };
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
