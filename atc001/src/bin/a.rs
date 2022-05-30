mod all_sides;

use crate::all_sides::all_sides;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
    }

    let c: Vec<Vec<char>> = (0..h)
        .map(|_| {
            proconio::input! {
                chars: Chars,
            }
            chars
        })
        .collect();

    let (s, g) = {
        let mut s = (0, 0);
        let mut g = (0, 0);
        (0..h).for_each(|i| {
            (0..w).for_each(|j| match &c[i][j] {
                's' => {
                    s = (i, j);
                }
                'g' => {
                    g = (i, j);
                }
                _ => {}
            });
        });
        (s, g)
    };

    let mut visited = vec![vec![false; w]; h];

    fn dfs(
        (i, j): (usize, usize),
        (h, w): (usize, usize),
        c: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        g: (usize, usize),
    ) {
        visited[i][j] = true;

        if visited[g.0][g.1] {
            return;
        }

        all_sides((i, j)).iter().for_each(|&(i, j)| {
            if i >= h || j >= w {
                return;
            }

            if c[i][j] == '#' {
                return;
            }

            if !visited[i][j] {
                dfs((i, j), (h, w), c, visited, g);
            }
        });
    }
    dfs(s, (h, w), &c, &mut visited, g);

    println!("{}", if visited[g.0][g.1] { "Yes" } else { "No" });
}
