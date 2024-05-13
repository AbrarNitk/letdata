use rand::{Rng, SeedableRng};

pub fn gen_rand<T: PartialOrd + rand::distributions::uniform::SampleUniform + Clone>(
    limit: usize,
    range: std::ops::Range<T>,
    seed: u64,
) -> Vec<T> {
    let mut generator = rand::rngs::StdRng::seed_from_u64(seed);
    let mut numbers = Vec::with_capacity(limit);

    // todo: ideally it should return an iterator also
    for _ in 0..limit {
        let number: T = generator.gen_range(range.clone());
        numbers.push(number);
    }
    numbers
}

#[cfg(test)]
mod test {
    #[test]
    fn with_len_i32() {
        let range = 500..600;
        let numbers = super::gen_rand(100, range.clone(), 10);
        for num in numbers.iter() {
            assert!(range.contains(num));
        }
        assert_eq!(numbers.len(), 100)
    }

    #[test]
    fn with_len_f32() {
        let range = 500.0..600.0f32;
        let numbers = super::gen_rand(100, range.clone(), 10);
        for num in numbers.iter() {
            assert!(range.contains(num));
        }
        assert_eq!(numbers.len(), 100)
    }
}
