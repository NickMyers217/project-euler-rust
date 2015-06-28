pub fn solve() {
	let mut res = 21;

	while !is_div_by_range(res, 20) {
		res += 1;
	}

	println!("{}", res);
}


fn is_div_by_range(num: u64, divisors: u64) -> bool {
	let mut res = true;

	for n in 2..divisors + 1 {
		if num % n != 0 {
			res = false;
			break;
		}
	}

	res
}