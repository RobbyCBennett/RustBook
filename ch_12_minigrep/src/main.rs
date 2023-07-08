mod args;
mod exit;
mod feedback;
mod find;
mod test;

use crate::args::*;
use crate::find::*;

fn main()
{
	// Parse arguments
	let arg_strings: Vec<String> = std::env::args().collect();
	let mut args = Args {
		option_ignore_case: false,
		pattern: &String::new(),
		files: Vec::<&String>::new(),
	};
	parse_args_or_exit(&arg_strings, &mut args);

	// If ignoring case, make pattern lowercase
	let lower_pattern;
	if args.option_ignore_case {
		lower_pattern = args.pattern.to_lowercase();
		args.pattern = &lower_pattern;
	}

	// Find pattern
	let exit_code = find_pattern(&args);
	std::process::exit(exit_code as i32);
}
