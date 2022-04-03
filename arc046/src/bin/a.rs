fn main() {
    proconio::input! {
        n: usize,
    }

    let quotient = n / 9;
    let remainder = n % 9;

    let digit = if remainder == 0 {
        quotient
    } else {
        quotient + 1
    };
    let number = if remainder == 0 { 9 } else { remainder };

    let answer = (0..digit).map(|_| number.to_string()).collect::<String>();

    println!("{}", answer);
}
