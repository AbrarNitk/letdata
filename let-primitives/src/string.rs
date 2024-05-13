use rand::{Rng, SeedableRng};

const ALPHABETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
fn gen_rand(len_range: std::ops::Range<usize>, seed: u64) -> String {
    let mut generator = rand::rngs::StdRng::seed_from_u64(seed);

    let string_len: usize = generator.gen_range(len_range);
    let string = (0..string_len)
        .map(|_| {
            let alphabet_idx = generator.gen_range(0..ALPHABETS.len());
            ALPHABETS.chars().nth(alphabet_idx).unwrap()
        })
        .collect::<String>();
    string
}


#[cfg(test)]
mod test {
    #[test]
    fn with_len() {
        let range = 500..600;
        let string = super::gen_rand(range.clone(), 10);
        assert!(range.contains(&string.len()));
    }
}