#[cfg(test)]
mod tests {

	use crate::exit::*;
	use crate::feedback::*;
	use crate::find::*;

	#[test]
	fn test_feedback()
	{
		missing(&String::from("missing.txt"));
		usage();
		help();
	}

	#[test]
	fn test_find()
	{
		let exit_code = find_pattern(&vec![
			String::from("minigrep"),
			String::from("needle"),
			String::from("test/haystack_0.txt"),
			String::from("test/haystack_1.txt"),
			String::from("test/haystack_2.txt"),
		]);
		assert_eq!(exit_code as i32, Exit::Good as i32);

		let exit_code = find_pattern(&vec![
			String::from("minigrep"),
			String::from("needle"),
			String::from("missing.txt"),
		]);
		assert_eq!(exit_code as i32, Exit::BadArg as i32);
	}

}
