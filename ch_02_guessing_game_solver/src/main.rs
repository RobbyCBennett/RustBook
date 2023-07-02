use rand::Rng;

const MAX_NUM: u128 = u128::MAX;
const MAX_TRIES: u128 = 1024;

#[cfg(debug_assertions)]
macro_rules! debug {
	($x:expr) => { println!($x) }
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
	($x:expr) => { std::convert::identity($x) }
}

fn main()
{
	println!("It's time for an exhilarating game... guess the number!");

	let mut low: u128 = 0;
	let mut guess: u128 = MAX_NUM >> 1;
	let mut high: u128 = MAX_NUM;

	let correct: u128 = rand::thread_rng().gen_range(0..=MAX_NUM);

	let start = std::time::Instant::now();

	for tries in 0..MAX_TRIES {
		debug!("\n{guess}");

		if guess < correct {
			debug!("Guess higher");
			low = guess;
		}
		else if guess > correct {
			debug!("Guess lower");
			high = guess;
		}
		else {
			let total = start.elapsed().as_nanos();
			let per = total / tries;
			println!("\n{total} ns total");
			println!("{per} ns per loop");
			println!("{tries} tries");
			println!("WINNER WINNER CHICKEN DINNER!");
			return;
		}
		guess = low + (high - low) / 2;
	}

	println!("\nYou give up???");
}
