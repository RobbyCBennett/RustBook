use crate::exit::*;
use crate::feedback::*;

pub struct Args<'a> {
	pub option_ignore_case: bool,
	pub pattern: &'a String,
	pub files: Vec<&'a String>,
}

pub fn parse_args_or_exit<'a> (arg_strings: &'a Vec<String>, args: &mut Args<'a>)
{
	// Parse arguments
	for arg_string in &arg_strings[1..] {
		// Option: help
		if arg_string == "--help" {
			help();
			std::process::exit(Exit::Good as i32);
		}
		// Option: ignore case
		else if arg_string == "-i" || arg_string == "--ignore-case" {
			args.option_ignore_case = true;
		}
		// Pattern
		else if args.pattern.len() == 0 {
			args.pattern = arg_string;
		}
		// Files
		else {
			args.files.push(arg_string);
		}
	}

	// Exit if there are no files
	if args.files.len() == 0 {
		usage();
		std::process::exit(Exit::Error as i32);
	}

	return
}
