pub fn solve() {
	for a in 1..501 {
		for b in 1..501 {
			for c in 1..501 {
				if a * a + b * b != c * c {
					continue;
				}

				if a + b + c != 1000 {
					continue;
				}

				println!("{} * {} * {} = {}", a, b, c, a * b * c);
			}
		}
	}
}