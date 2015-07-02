pub struct Sieve {
	pub primes: Vec<u64>,
	pub contents: Vec<bool>
}


impl Sieve {
	pub fn new(size: usize) -> Sieve {
		let mut s = Sieve { 
			primes: Vec::with_capacity(size),
			contents: vec![true; size]
		};
		s.contents[0] = false;
		s.contents[1] = false;
		s
	}	

	pub fn find_primes(&mut self, amount: usize) {
		for i in 0..self.contents.len() {
			if self.primes.len() == amount && amount != 0 {
				break;
			}

			if self.contents[i] {
				self.primes.push(i as u64);

				let mut multiple = 2;
				while i * multiple < self.contents.len() {
					self.contents[i * multiple] = false;
					multiple += 1;
				}
			}
		}
	}

	pub fn sum_primes(&self) -> u64 {
		*( &self.primes.iter().fold(0, |acc, n| acc + n) )
	}

	pub fn print_primes(&self) {
		for p in &self.primes {
			println!("{}", p);
		}
	}
}
