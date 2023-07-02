macro_rules! error {
	($a:expr) => {
		println!($a);
		return false;
	};
}

use rand::Rng;
use std::io;

fn read_line(string: &mut String) -> bool {
	let result = io::stdin().read_line(string);

	if result.is_err() {
		return false;
	}

	// Remove new line
	if string.ends_with('\n') {
		string.pop();
		if string.ends_with('\r') {
			string.pop();
		}
	}

	return true;
}

fn get_correct_num(correct_num: u128) -> bool
{
	println!();

	// Get number
	let mut guess_str = String::new();
	if !read_line(&mut guess_str) {
		error!("Guess a u128 this time");
	}

	// Parse number
	let parse_result = guess_str.parse::<u128>();
	if parse_result.is_err() {
		error!("Guess a u128 this time");
	}
	let guess_num: u128 = parse_result.unwrap();

	// Compare number
	if guess_num < correct_num {
		error!("Guess higher");
	}
	if guess_num > correct_num {
		error!("Guess lower");
	}

	println!("WINNER WINNER CHICKEN DINNER!");
	return true;
}

fn main()
{
	println!("It's time for an exhilarating game... guess the number!");

	let correct_num: u128 = rand::thread_rng().gen_range(0..=u128::MAX);

	while !get_correct_num(correct_num) {

	}
}
