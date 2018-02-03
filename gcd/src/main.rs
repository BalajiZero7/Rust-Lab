fn main() {

    let a: u32 = gcd(46, 12);
    println!("The GCD of 23 and 14 is {}", a);
}

fn gcd(mut m: u32, mut n:u32) ->u32 {

assert!(m != 0 && n != 0);
while m != 0 {
    if m < n {
    let t = m;
        m = n;
        n = t
    }
    m = m % n; 
}
n
}