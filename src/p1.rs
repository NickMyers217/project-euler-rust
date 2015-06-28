pub fn solve() {
	let mut sum = 0;

	for n in (1..1000) {
		if div(n, 5) || div(n, 3) {
			sum += n;
		}
	}

	println!("{}", sum);
}

fn div(a: u32, b: u32) -> bool {
	a % b == 0
}