use util::primes;

pub fn solve() {
	let mut s = primes::Sieve::new(2000000);
	s.find_primes(0);
	println!("{}", s.sum_primes());
}