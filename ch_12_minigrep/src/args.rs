use crate::exit::*;
use crate::feedback::*;

pub fn get_args_or_exit() -> Vec<String>
{
	// Get arguments
	let args: Vec<String> = std::env::args().collect();

	// Exit if user wants help
	for arg in &args {
		if arg == "--help" {
			help();
			std::process::exit(Exit::Good as i32);
		}
	}

	// Exit if not enough arguments
	if args.len() < 3 {
		usage();
		std::process::exit(Exit::BadArg as i32);
	}

	return args;
}
