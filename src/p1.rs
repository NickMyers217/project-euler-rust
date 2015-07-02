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

pub fn solve2() {
	println!("{}",
		(1..1000)
		.filter(|n| n % 3 == 0 || n % 5 == 0)
		.collect::<Vec<u32>>()
		.iter()
		.fold(0, |acc, n| acc + n)
	);
}