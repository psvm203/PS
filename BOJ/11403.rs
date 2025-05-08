use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let n = read_ints()[0];
    let mut graph: Vec<_> = (0..n).map(|_| read_ints()).collect();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                graph[i][j] |= graph[i][k] & graph[k][j];
            }
        }
    }

    let output = graph
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    print!("{output}");
}
