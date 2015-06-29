mod util;

use util::primes::Sieve;

fn main() {
	let num_primes = 10001;
	let mut s = Sieve::new(num_primes * num_primes + 1);
	s.find_primes(num_primes);
	s.print_primes();
}	
