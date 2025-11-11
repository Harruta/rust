use std::io::{self, BufRead};

fn gcd(mut a: i64, mut b: i64)-> i64 {
    while  b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let line: String = lines.next().unwrap().unwrap();
        let a: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if a.is_empty(){
        println!("0");
        continue;
    }
    let mut g: i64 = a[0];
    for &val in a.iter().skip(1){
        g = gcd(g,val);
        if g == 1{
            break;
        }
    }
    if g == 1{
        println!("0");
        continue;
    }
    let mut ans: i64 = 0;
    if gcd(g,n) == 1{
        ans = 1;
    } else if n > 1 && gcd(g,n - 1) == 1{
        ans = 2;
    } else {
        ans = 3;
    }
    println!("{}", ans);
 }
}
