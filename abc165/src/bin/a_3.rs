fn main() {
    proconio::input! {
        k: usize,
        a: usize,
        b: usize,
    }

    let mut answer = false;

    let aa = a / k;
    let bb = b / k;

    if aa < bb {
        answer = true;
    }

    if a % k == 0 {
        answer = true;
    }

    println!("{}", if answer { "OK" } else { "NG" });
}
