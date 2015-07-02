pub fn solve() {
	println!("{}", fib(1, 1, 0, 4000000));
}

fn fib(a: u32, b: u32, sum: u32, cap: u32) -> u32 {
	let c = a + b;

	if c > cap
	{
		sum
	} 
	else
	{
		let mut next_sum = sum;
		
		if c % 2 == 0 { next_sum += c; }

		fib(b, c, next_sum, cap)
	}
}
