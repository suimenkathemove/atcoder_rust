use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    proconio::input! {
        R: usize,
        C: usize,
        mut sy: usize,
        mut sx: usize,
        mut gy: usize,
        mut gx: usize,
    }

    sy -= 1;
    sx -= 1;
    gy -= 1;
    gx -= 1;

    let c: Vec<Vec<char>> = (0..R)
        .map(|_| {
            proconio::input! {
                chars: Chars,
            }
            chars
        })
        .collect();

    let mut distance = vec![vec![0_usize; C]; R];

    let mut q = VecDeque::<(usize, usize)>::new();

    let mut visited = vec![vec![false; C]; R];

    q.push_back((sy, sx));

    visited[sy][sx] = true;

    while !q.is_empty() {
        let (y, x) = q.pop_front().unwrap();

        [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
            .iter()
            .for_each(|&(y2, x2)| {
                if c[y2][x2] == '#' {
                    return;
                }

                if !visited[y2][x2] {
                    distance[y2][x2] = distance[y][x] + 1;

                    q.push_back((y2, x2));

                    visited[y2][x2] = true;
                }
            });
    }

    println!("{}", distance[gy][gx]);
}
