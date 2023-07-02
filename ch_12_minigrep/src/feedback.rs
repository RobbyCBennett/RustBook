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
	println!("  -E, --extended-regexp     PATTERN is an extended regular expression (ERE)");
	println!("  -F, --fixed-strings       PATTERN is a set of newline-separated fixed strings");
	println!("  -G, --basic-regexp        PATTERN is a basic regular expression (BRE)");
	println!("  -P, --perl-regexp         PATTERN is a Perl regular expression");
	println!("  -e, --regexp=PATTERN      use PATTERN for matching");
	println!("  -f, --file=FILE           obtain PATTERN from FILE");
	println!("  -i, --ignore-case         ignore case distinctions");
	println!("  -w, --word-regexp         force PATTERN to match only whole words");
	println!("  -x, --line-regexp         force PATTERN to match only whole lines");
	println!("  -z, --null-data           a data line ends in 0 byte, not newline");
	println!("");
	println!("Miscellaneous:");
	println!("  -s, --no-messages         suppress error messages");
	println!("  -v, --invert-match        select non-matching lines");
	println!("  -V, --version             print version information and exit");
	println!("      --help                display this help and exit");
	println!("      --mmap                use memory-mapped input if possible");
	println!("");
	println!("Output control:");
	println!("  -m, --max-count=NUM       stop after NUM matches");
	println!("  -b, --byte-offset         print the byte offset with output lines");
	println!("  -n, --line-number         print line number with output lines");
	println!("      --line-buffered       flush output on every line");
	println!("  -H, --with-filename       print the filename for each match");
	println!("  -h, --no-filename         suppress the prefixing filename on output");
	println!("      --label=LABEL         print LABEL as filename for standard input");
	println!("  -o, --only-matching       show only the part of a line matching PATTERN");
	println!("  -q, --quiet, --silent     suppress all normal output");
	println!("      --binary-files=TYPE   assume that binary files are TYPE;");
	println!("                            TYPE is `binary', `text', or `without-match'");
	println!("  -a, --text                equivalent to --binary-files=text");
	println!("  -I                        equivalent to --binary-files=without-match");
	println!("  -d, --directories=ACTION  how to handle directories;");
	println!("                            ACTION is `read', `recurse', or `skip'");
	println!("  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;");
	println!("                            ACTION is `read' or `skip'");
	println!("  -R, -r, --recursive       equivalent to --directories=recurse");
	println!("      --include=FILE_PATTERN  search only files that match FILE_PATTERN");
	println!("      --exclude=FILE_PATTERN  skip files and directories matching FILE_PATTERN");
	println!("      --exclude-from=FILE   skip files matching any file pattern from FILE");
	println!("      --exclude-dir=PATTERN directories that match PATTERN will be skipped.");
	println!("  -L, --files-without-match print only names of FILEs containing no match");
	println!("  -l, --files-with-matches  print only names of FILEs containing matches");
	println!("  -c, --count               print only a count of matching lines per FILE");
	println!("  -T, --initial-tab         make tabs line up (if needed)");
	println!("  -Z, --null                print 0 byte after FILE name");
	println!("");
	println!("Context control:");
	println!("  -B, --before-context=NUM  print NUM lines of leading context");
	println!("  -A, --after-context=NUM   print NUM lines of trailing context");
	println!("  -C, --context=NUM         print NUM lines of output context");
	println!("  -NUM                      same as --context=NUM");
	println!("      --color[=WHEN],");
	println!("      --colour[=WHEN]       use markers to highlight the matching strings;");
	println!("                            WHEN is `always', `never', or `auto'");
	println!("  -U, --binary              do not strip CR characters at EOL (MSDOS)");
	println!("  -u, --unix-byte-offsets   report offsets as if CRs were not there (MSDOS)");
	println!("");
	println!("`eminigrep' means `minigrep -E'.  `fminigrep' means `minigrep -F'.");
	println!("Direct invocation as either `eminigrep' or `fminigrep' is deprecated.");
	println!("With no FILE, or when FILE is -, read standard input.  If less than two FILEs");
	println!("are given, assume -h.  Exit status is 0 if any line was selected, 1 otherwise;");
	println!("if any error occurs and -q was not given, the exit status is 2.");
}
