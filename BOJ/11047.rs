use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let mut numbers = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let target = numbers.nth(1).unwrap();
    let coins = numbers;

    let min_coins_count: usize = coins
        .rev()
        .scan(target, |acc, coin| {
            let coin_count = *acc / coin;
            *acc -= coin * coin_count;
            Some(coin_count)
        })
        .sum();

    print!("{min_coins_count}");
}
