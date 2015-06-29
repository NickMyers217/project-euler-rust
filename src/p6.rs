pub fn solve() {
	println!("{}", square_sum(100) - sum_squares(100));
}

fn sum_squares(cap: u32) -> u32 {
	let mut res = 0;

	for n in 1..cap + 1 {
		res += n * n;
	}

	res
}

fn square_sum(cap: u32) -> u32 {
	let mut res = 0;

	for n in 1..cap + 1 {
		res += n;
	}

	res * res
}