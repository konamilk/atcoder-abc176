use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input!{
        n_as_chars: Chars
    }

    let n = n_as_chars.iter().map(|&x| x as i32 - 48).collect::<Vec<i32>>();

    if n.iter().sum::<i32>() % 9 == 0{
        println!("Yes");
    }
    else {
        println!("No");
    }

}
