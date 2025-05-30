use std::io::*;

fn main() {
    let stdin = read_to_string(stdin()).unwrap();
    let mut lines = stdin.lines();

    let mut read_ints = || -> Vec<usize> {
        lines
            .next()
            .unwrap()
            .split(' ')
            .flat_map(|x| x.parse())
            .collect()
    };

    let test_cases_count = read_ints()[0];

    let mut is_cycle = [false; 100000];
    let mut visited = [false; 100000];

    let mut solve_test_case = || {
        let n = read_ints()[0];
        let graph = read_ints();

        is_cycle[0..n].fill(false);
        visited[0..n].fill(false);

        let mut outsider = n;

        for begin in 0..n {
            if is_cycle[begin] {
                continue;
            }

            let mut cur = begin;
            let mut visits = vec![];

            while !is_cycle[cur] {
                is_cycle[cur] = true;
                visits.push(cur);
                cur = graph[cur] - 1;
            }

            if !visited[cur] {
                let mut cnt = 0;
                let mut flag = false;

                for &visit in &visits {
                    if visit == cur {
                        flag = true;
                        cnt += 1;
                        continue;
                    }

                    if flag {
                        cnt += 1
                    }
                }

                outsider -= cnt;
            }

            for &visit in &visits {
                visited[visit] = false;
            }
        }

        outsider
    };

    let output = (0..test_cases_count)
        .map(|_| solve_test_case().to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{output}");
}
