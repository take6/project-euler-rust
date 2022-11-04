use std::cmp;

use primes;

fn main() {
    let number: u64 = 600851475143;

    let factors: Vec<u64> = primes::factors(number);

    let mut max_factor: u64 = 0;
    let mut verify: u64 = 1;

    println!("Number {}: found {} factors", number, factors.len());
    for x in factors {
        println!("  {}", x);
        max_factor = cmp::max(x, max_factor);
        verify *= x;
    }
    println!("Max prime factor is {}", max_factor);
    assert!(number == verify);
}
