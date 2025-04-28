use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let n: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let pokemons = lines.clone().take(n);
    let queries = lines.skip(n);

    let mut index_to_name = vec![""; n + 1];
    let mut name_to_index = HashMap::with_capacity(n);

    pokemons.enumerate().for_each(|(i, pokemon)| {
        index_to_name[i + 1] = pokemon;
        name_to_index.insert(pokemon, i + 1);
    });

    let process_query = |query: &str| match query.parse::<usize>() {
        Ok(index) => String::from(index_to_name[index]),
        Err(_) => name_to_index[query].to_string(),
    };

    let output = queries.map(process_query).collect::<Vec<_>>().join("\n");

    print!("{output}");
}
