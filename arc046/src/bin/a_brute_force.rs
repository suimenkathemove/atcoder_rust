fn main() {
    proconio::input! {
        n: u32,
    }

    let mut count = 0;

    for i in 1..=555555 {
        let is_doublet = i
            .to_string()
            .chars()
            .all(|c| c == i.to_string().chars().next().unwrap());
        if is_doublet {
            count += 1;
        }

        if count == n {
            println!("{}", i);
            break;
        }
    }
}
