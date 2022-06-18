use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
    }

    let a: Vec<Vec<char>> = (0..n)
        .map(|_| {
            proconio::input! {
                chars: Chars,
            }
            chars
        })
        .collect();

    let group = {
        let mut group = vec![vec![]; 11];
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                let c = a[i][j];
                let index = match c {
                    'S' => 0,
                    'G' => 10,
                    _ => c.to_digit(10).unwrap() as usize,
                };
                group[index].push((i, j));
            });
        });
        group
    };

    let max = std::isize::MAX;
    let mut cost = vec![vec![max; m]; n];

    let (si, sj) = group[0][0];
    cost[si][sj] = 0;

    for n in 1..=10 {
        for &(i, j) in &group[n] {
            for &(i2, j2) in &group[n - 1] {
                if cost[i2][j2] == max {
                    break;
                }

                cost[i][j] = cost[i][j].min(
                    cost[i2][j2]
                        + (i as isize - i2 as isize).abs()
                        + (j as isize - j2 as isize).abs(),
                );
            }
        }
    }

    let (gi, gj) = group[10][0];
    println!(
        "{}",
        if cost[gi][gj] == max {
            -1
        } else {
            cost[gi][gj]
        }
    );
}
