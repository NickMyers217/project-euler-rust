pub fn solve() {
	let tri_nums =
		(1..)
		.map(|i| get_tri_num(i))
		.take_while(|i| get_num_divisors(*i) <= 500)
		.collect::<Vec<u64>>();

	println!("{}", get_tri_num(tri_nums.len() as u32 + 1));
}

fn get_tri_num(i: u32) -> u64 {
	(1..i + 1).fold(0, |acc, n| acc + n) as u64
}

fn get_num_divisors(i: u64) -> u32 {
	(1..(i as f64).sqrt() as u64 + 1)
	.filter(|d| i % d == 0)
	.collect::<Vec<_>>()
	.len() as u32 * 2
}
