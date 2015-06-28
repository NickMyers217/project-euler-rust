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
	let forward: Vec<char> = w.chars().collect();
	let backward: Vec<char> = w.chars().rev().collect();

	for i in 0..w.len() {
		if forward[i] != backward[i] {
			res = false;
		}
	}

	res
}

