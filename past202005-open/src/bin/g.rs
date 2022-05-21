use std::collections::VecDeque;

fn main() {
    proconio::input! {
        N: usize,
        X: isize,
        Y: isize,
        xy: [(isize, isize); N],
    }

    const OFFSET: usize = 201;
    const MAX_INDEX: usize = 201 + OFFSET;

    let X = (X + OFFSET as isize) as usize;
    let Y = (Y + OFFSET as isize) as usize;
    let xy: Vec<(usize, usize)> = xy
        .iter()
        .map(|(x, y)| {
            (
                (x + OFFSET as isize) as usize,
                (y + OFFSET as isize) as usize,
            )
        })
        .collect();

    let s = (0 + OFFSET, 0 + OFFSET);

    let mut d = [[-1; MAX_INDEX + 1]; MAX_INDEX + 1];

    let mut q = VecDeque::<(usize, usize)>::new();

    d[s.1][s.0] = 0;

    q.push_back(s);

    while let Some((x, y)) = q.pop_front() {
        let all_sides = {
            let mut all_sides = vec![(x + 1, y + 1), (x, y + 1), (x + 1, y)];

            if x > 0 {
                all_sides.push((x - 1, y + 1));
                all_sides.push((x - 1, y));
            }

            if y > 0 {
                all_sides.push((x, y - 1));
            }

            all_sides
        };

        all_sides.iter().for_each(|&(x2, y2)| {
            if x2 > MAX_INDEX || y2 > MAX_INDEX {
                return;
            }

            if xy.contains(&(x2, y2)) {
                return;
            }

            if d[y2][x2] == -1 {
                d[y2][x2] = d[y][x] + 1;

                q.push_back((x2, y2));
            }
        });
    }

    println!("{}", d[Y][X]);
}
