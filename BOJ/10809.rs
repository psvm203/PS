use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let s = input.trim();

    let mut first_indices = [-1; 26];

    for (i, ch) in s.chars().enumerate() {
        let index = ch as usize - 'a' as usize;

        if first_indices[index] == -1 {
            first_indices[index] = i as i32;
        }
    }

    let indices = first_indices.map(|index| index.to_string()).join(" ");
    print!("{indices}");
}
