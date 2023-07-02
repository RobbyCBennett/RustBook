use crate::exit::*;
use crate::feedback::*;

pub fn find_pattern(args: &Vec<String>) -> Exit
{
	let mut exit_code = Exit::Good;

	// Split arguments
	let pattern = &args[1];
	let files = &args[2..];

	// Read each file
	let multiple_files = files.len() > 1;
	for file in files {
		let result = std::fs::read_to_string(file);

		// Error and skip file if missing
		if result.is_err() {
			exit_code = Exit::BadArg;
			missing(&file);
			continue;
		}

		// Print each line with the pattern
		for line in result.unwrap().split("\n") {
			if line.contains(pattern) {
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
