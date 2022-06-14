use std::collections::VecDeque;

fn main() {
    proconio::input! {
        k: usize,
    }

    let mut n = 0;

    let mut q = VecDeque::<usize>::new();

    (1..=9).for_each(|i| {
        q.push_back(i);
    });

    while let Some(i) = q.pop_front() {
        n += 1;

        if n >= k {
            println!("{}", i);
            break;
        }

        let i_last = i % 10;

        if i_last > 0 {
            q.push_back(format!("{}{}", i, i_last - 1).parse().unwrap());
        }
        q.push_back(format!("{}{}", i, i_last).parse().unwrap());
        if i_last < 9 {
            q.push_back(format!("{}{}", i, i_last + 1).parse().unwrap());
        }
    }
}
