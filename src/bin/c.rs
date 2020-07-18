#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut b: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if i % 2 == 1 {
            b.push_back(a[i]);
        } else {
            b.push_front(a[i]);
        }
    }
    if n % 2 == 0 {
        for i in 0..n {
            print!("{} ", b[n - 1 - i]);
        }
    } else {
        for i in 0..n {
            print!("{} ", b[i]);
        }
    }
}
