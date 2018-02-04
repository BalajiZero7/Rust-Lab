fn is_prime(n: u64) -> bool {
    match n {
        0...1 => false,
        _ => (!(2..n).any(|d| n % d == 0)),
}
}

fn main() {
    let n = 6;
    let a = is_prime(n);
    println!("Is {} given number is a prime number? : {}", n, a);  
}