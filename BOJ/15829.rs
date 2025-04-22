use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let text = input.lines().nth(1).unwrap();

    fn mod_pow(base: usize, modulus: usize, x: usize) -> usize {
        if x == 0 {
            1
        } else if x % 2 == 0 {
            let sqrt = mod_pow(base, modulus, x / 2) % modulus;
            sqrt * sqrt % modulus
        } else {
            mod_pow(base, modulus, x - 1) % modulus * base % modulus
        }
    }

    let base = 31;
    let modulus = 1234567891;

    let hash_char =
        |(i, ch)| mod_pow(base, modulus, i) % modulus * (ch as usize - 'a' as usize + 1) % modulus;

    let hash: usize = text
        .chars()
        .enumerate()
        .map(|(i, ch)| hash_char((i, ch)))
        .fold(0, |acc, x| (acc + x) % modulus);

    print!("{hash}");
}
