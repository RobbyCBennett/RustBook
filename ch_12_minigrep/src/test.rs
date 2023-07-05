#[cfg(test)]
mod tests {

	use crate::args::*;
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
	fn test_parse_args_or_exit_0()
	{
		let arg_strings = vec![
			String::from("minigrep"),
			String::from("needle"),
			String::from("test/haystack_0.txt"),
			String::from("test/haystack_1.txt"),
			String::from("test/haystack_2.txt"),
		];
		let mut args = Args {
			option_ignore_case: false,
			pattern: &String::new(),
			files: Vec::<&String>::new(),
		};

		parse_args_or_exit(&arg_strings, &mut args);

		assert_eq!(args.option_ignore_case, false);
		assert_eq!(args.pattern, &String::from("needle"));
		assert_eq!(args.files.len(), 3);
		assert_eq!(args.files, vec![
			&String::from("test/haystack_0.txt"),
			&String::from("test/haystack_1.txt"),
			&String::from("test/haystack_2.txt"),
		]);
	}

	#[test]
	fn test_parse_args_or_exit_1()
	{
		let arg_strings = vec![
			String::from("minigrep"),
			String::from("-i"),
			String::from("needle"),
			String::from("test/haystack_0.txt"),
			String::from("test/haystack_1.txt"),
			String::from("test/haystack_2.txt"),
		];
		let mut args = Args {
			pattern: &String::new(),
			files: Vec::<&String>::new(),
			option_ignore_case: false,
		};

		parse_args_or_exit(&arg_strings, &mut args);

		assert_eq!(args.option_ignore_case, true);
		assert_eq!(args.pattern, &String::from("needle"));
		assert_eq!(args.files.len(), 3);
		assert_eq!(args.files, vec![
			&String::from("test/haystack_0.txt"),
			&String::from("test/haystack_1.txt"),
			&String::from("test/haystack_2.txt"),
		]);
	}

	#[test]
	fn test_parse_args_or_exit_2()
	{
		let arg_strings = vec![
			String::from("minigrep"),
			String::from("needle"),
			String::from("-i"),
			String::from("test/haystack_0.txt"),
			String::from("test/haystack_1.txt"),
			String::from("test/haystack_2.txt"),
		];
		let mut args = Args {
			pattern: &String::new(),
			files: Vec::<&String>::new(),
			option_ignore_case: false,
		};

		parse_args_or_exit(&arg_strings, &mut args);

		assert_eq!(args.option_ignore_case, true);
		assert_eq!(args.pattern, &String::from("needle"));
		assert_eq!(args.files.len(), 3);
		assert_eq!(args.files, vec![
			&String::from("test/haystack_0.txt"),
			&String::from("test/haystack_1.txt"),
			&String::from("test/haystack_2.txt"),
		]);
	}

	#[test]
	fn test_parse_args_or_exit_3()
	{
		let arg_strings = vec![
			String::from("minigrep"),
			String::from("needle"),
			String::from("test/haystack_0.txt"),
			String::from("test/haystack_1.txt"),
			String::from("-i"),
			String::from("test/haystack_2.txt"),
		];
		let mut args = Args {
			pattern: &String::new(),
			files: Vec::<&String>::new(),
			option_ignore_case: false,
		};

		parse_args_or_exit(&arg_strings, &mut args);

		assert_eq!(args.option_ignore_case, true);
		assert_eq!(args.pattern, &String::from("needle"));
		assert_eq!(args.files.len(), 3);
		assert_eq!(args.files, vec![
			&String::from("test/haystack_0.txt"),
			&String::from("test/haystack_1.txt"),
			&String::from("test/haystack_2.txt"),
		]);
	}

	#[test]
	fn test_parse_args_or_exit_4()
	{
		let arg_strings = vec![
			String::from("minigrep"),
			String::from("needle"),
			String::from("test/haystack_0.txt"),
			String::from("test/haystack_1.txt"),
			String::from("test/haystack_2.txt"),
			String::from("-i"),
		];
		let mut args = Args {
			pattern: &String::new(),
			files: Vec::<&String>::new(),
			option_ignore_case: false,
		};

		parse_args_or_exit(&arg_strings, &mut args);

		assert_eq!(args.option_ignore_case, true);
		assert_eq!(args.pattern, &String::from("needle"));
		assert_eq!(args.files.len(), 3);
		assert_eq!(args.files, vec![
			&String::from("test/haystack_0.txt"),
			&String::from("test/haystack_1.txt"),
			&String::from("test/haystack_2.txt"),
		]);
	}

	#[test]
	fn test_find_0()
	{
		let file_0 = String::from("test/haystack_0.txt");
		let file_1 = String::from("test/haystack_1.txt");
		let file_2 = String::from("test/haystack_2.txt");
		let args = Args {
			option_ignore_case: false,
			pattern: &String::from("needle"),
			files: vec![
				&file_0,
				&file_1,
				&file_2,
			],
		};

		let exit_code = find_pattern(&args);

		assert_eq!(exit_code as i32, Exit::Good as i32);
	}

	#[test]
	fn test_find_1()
	{
		let file = String::from("missing");
		let args = Args {
			option_ignore_case: false,
			pattern: &String::from("needle"),
			files: vec![
				&file,
			],
		};

		let exit_code = find_pattern(&args);

		assert_eq!(exit_code as i32, Exit::BadArg as i32);
	}

}
