fn main() {
    proconio::input! {
        n: usize,
        m: usize,
    }

    let edges = {
        let mut edges = vec![vec![]; n];
        (0..m).for_each(|_| {
            proconio::input! {
                (x, y): (usize, usize),
            }
            let x = x - 1;
            let y = y - 1;
            edges[x].push(y);
        });
        edges
    };

    let mut length = vec![0; n];

    let mut done = vec![false; n];

    let starts = {
        let mut starts = vec![true; n];
        (0..n).for_each(|i| {
            edges[i].iter().for_each(|&j| {
                starts[j] = false;
            });
        });
        starts
    };

    fn rec(
        i: usize,
        edges: &Vec<Vec<usize>>,
        length: &mut Vec<usize>,
        done: &mut Vec<bool>,
    ) -> usize {
        if done[i] {
            return length[i];
        }

        edges[i].iter().for_each(|&j| {
            length[i] = length[i].max(rec(j, edges, length, done) + 1);
        });

        done[i] = true;

        length[i]
    }

    (0..n).for_each(|i| {
        if starts[i] {
            rec(i, &edges, &mut length, &mut done);
        }
    });

    println!("{}", length.iter().max().unwrap());
}
