use crate::args::*;
use crate::exit::*;
use crate::feedback::*;

pub fn find_pattern(args: &Args) -> Exit
{
	let mut exit_code = Exit::NotFound;

	// Read each file
	let multiple_files = args.files.len() > 1;
	for file in &args.files {
		let result = std::fs::read_to_string(file);

		// Error and skip file if missing
		if result.is_err() {
			exit_code = Exit::Error;
			missing(&file);
			continue;
		}

		// Read each line
		for mut line in result.unwrap().split("\n") {
			// If ignoring case, make line lowercase
			let lower_line;
			if args.option_ignore_case {
				lower_line = line.to_lowercase();
				line = &lower_line;
			}

			// Find each line with the pattern
			if line.contains(args.pattern) {
				exit_code = Exit::Good;
				if multiple_files {
					println!("{file}:{line}");
				}
				else {
					println!("{line}");
				}
			}
		}
	}

	return exit_code;
}
