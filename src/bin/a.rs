use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input!{
        n: i32,
        x: i32,
        t: i32
    }

    let kaisu = ((n as f32) / (x as f32)).ceil() as i32;

    println!("{}", kaisu * t);
}
