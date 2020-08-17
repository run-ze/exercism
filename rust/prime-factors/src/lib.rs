pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut result: Vec<u64> = Vec::new();

    if n == 2 {
        return vec![n];
    }

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            result.push(i);
            n /= i
        }
        i += 1
    }

    if n > 2 {
        result.push(n)
    }

    result
}
