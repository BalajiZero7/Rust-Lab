fn is_prime(n: u64) -> bool {
    match n {
        0...1 => false,
        _ => (!(2..n).any(|d| n % d == 0)),
   
}
}

fn main() {
    let a = is_prime(6);
    println!("Is the given number is a prime number? : {}", a);  
}