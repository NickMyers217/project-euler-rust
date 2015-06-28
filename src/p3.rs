pub fn solve() {
	let mut num: u64 = 600851475143;
	let mut n: u64 = 2;

	while n <= num {
		if num % n == 0 {
			println!("{}", n);
			num /= n;
			n = 2;
		}

		n += 1;
	}
}