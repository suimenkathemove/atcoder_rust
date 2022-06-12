fn main() {
    proconio::input! {
        n: usize,
        h: [isize; n],
    }

    let mut cost = vec![0; n];

    let mut done = vec![false; n];

    fn rec(i: usize, h: &Vec<isize>, cost: &mut Vec<isize>, done: &mut Vec<bool>) -> isize {
        if done[i] {
            return cost[i];
        }

        cost[i] = match i {
            0 => 0,
            1 => (h[0] - h[1]).abs(),
            _ => std::cmp::min(
                rec(i - 1, h, cost, done) + (h[i - 1] - h[i]).abs(),
                rec(i - 2, h, cost, done) + (h[i - 2] - h[i]).abs(),
            ),
        };

        done[i] = true;

        cost[i]
    }

    println!("{}", rec(n - 1, &h, &mut cost, &mut done));
}
