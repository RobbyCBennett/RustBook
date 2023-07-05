pub fn missing(file: &String)
{
	eprintln!("minigrep: {file}: No such file or directory");
}

pub fn usage()
{
	eprintln!("Usage: minigrep [OPTION]... PATTERN [FILE]...");
	eprintln!("Try `minigrep --help' for more information.");
}

pub fn help()
{
	println!("Usage: minigrep [OPTION]... PATTERN [FILE]...");
	println!("Search for PATTERN in each FILE or standard input.");
	println!("PATTERN is, by default, a basic regular expression (BRE).");
	println!("Example: minigrep -i 'hello world' menu.h main.c");
	println!("");
	println!("Regexp selection and interpretation:");
	println!("  -i, --ignore-case         ignore case distinctions");
	println!("");
	println!("Miscellaneous:");
	println!("      --help                display this help and exit");
	println!("");
	println!("Exit status is 0 if any line was selected, 1 otherwise;");
	println!("if any error occurs, the exit status is 2.");
}
