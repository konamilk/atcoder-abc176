use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        n:usize,
        a: [i64; n]
    }

    let mut dp = vec![];

    dp.push(a[0]);

    for i in 1..n {
        dp.push(std::cmp::max(dp[i - 1], a[i]))
    }

    let ans = dp.iter().sum::<i64>() - a.iter().sum::<i64>();

    println!("{}", ans)
}
