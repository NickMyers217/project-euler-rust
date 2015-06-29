use util::primes::Sieve;

pub fn solve() {
	let num_primes = 10001;
	let mut s = Sieve::new(num_primes * num_primes + 1);
	s.find_primes(num_primes);
	s.print_primes();
}