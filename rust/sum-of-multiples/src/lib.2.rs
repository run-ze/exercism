struct F {
    factor: u32,
    next: u32,
}

impl F {
    fn new(f: u32) -> F {
        F { factor: f, next: 0 }
    }
}

impl Iterator for F {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.factor == 0 {
            None
        } else {
            self.next = self.next + self.factor;
            Some(self.next)
        }
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut eligible: Vec<u32> = factors
        .iter()
        .flat_map(|f| F::new(*f).take_while(|&x| x < limit))
        .collect();

    eligible.sort();
    eligible.dedup();
    eligible.iter().sum()
}
