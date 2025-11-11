use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let t:usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line: String = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: u64 = parts.next().unwrap().parse().unwrap();
        let b: u64 = parts.next().unwrap().parse().unwrap();
        let k: u64 = parts.next().unwrap().parse().unwrap();
        let g = gcd(a,b);
        let pa = a / g;
        let pb = b / g;
        let cost = if pa > k || pb > k { 2 } else { 1 };
        println!("{}",cost);
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64{
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
