use crate::args::*;
use crate::find::*;

mod args;
mod exit;
mod feedback;
mod find;
mod test;

fn main()
{
	let arg_strings: Vec<String> = std::env::args().collect();
	let mut args = Args {
		option_ignore_case: false,
		pattern: &String::new(),
		files: Vec::<&String>::new(),
	};
	parse_args_or_exit(&arg_strings, &mut args);

	let exit_code = find_pattern(&args);
	std::process::exit(exit_code as i32);
}
