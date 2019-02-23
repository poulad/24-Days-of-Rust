extern crate primal;

use primal::Sieve;

fn main() {
    let sieve = Sieve::new(10000);
    let suspect = 5273;
    println!("Is {} prime? {}", suspect, sieve.is_prime(suspect));

    let not_a_prime = 1024;
    dbg!(sieve.is_prime(not_a_prime));

    let n = 1000;
    println!("finding the {}th prime number...", n);
    match sieve.primes_from(0).nth(n - 1) {
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("idk!"),
    }
}
