pub fn solve() {
	let mut largest = 0;

	for a in 100..1000 {
		for b in 100..1000 {
			let num = a * b;
			if is_palindrome(&num.to_string()) && num > largest {
				largest = num;
			}
		}
	}

	println!("{}", largest);
}

fn is_palindrome(w: &str) -> bool {
	let mut res = true;

	// The program runs slow
	// I suspect these lines are the cause
	// I think these vector allocations probably need to be reserved in some way, not sure
	// But I couldn't find an easier way to reverse all the characters in a string
	// Thank you for having tons of outdated and incorrect docs because you change all the time rust
	let forward: Vec<char> = w.chars().collect();
	let backward: Vec<char> = w.chars().rev().collect();

	for i in 0..w.len() {
		if forward[i] != backward[i] {
			res = false;
		}
	}

	res
}

