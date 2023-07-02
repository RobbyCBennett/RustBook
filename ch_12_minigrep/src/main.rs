use crate::args::*;
use crate::find::*;

mod args;
mod exit;
mod feedback;
mod find;
mod test;

fn main()
{
	let args = get_args_or_exit();
	let exit_code = find_pattern(&args);
	std::process::exit(exit_code as i32);
}
