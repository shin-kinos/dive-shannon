
use std::env;
use std::process;

pub struct Options {
	pub input    : String,
	pub output   : String,
	pub color    : String,
	pub tolerate : String,
}

impl Options {
	pub fn new() -> Options
	{
		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_o : &String = &String::new();
		let mut arg_c : &String = &String::from( "no" );
		let mut arg_t : &String = &String::from( "yes" );

		if argc < 5 { show_usage(&argv[0]) };

		let mut i : usize = 1;
		while i < argc {
			match argv[i].as_str() {
				"-i" => { i += 1; arg_i = &argv[i]; },
				"-o" => { i += 1; arg_o = &argv[i]; },
				"-c" => { i += 1; arg_c = &argv[i]; },
				"-t" => { i += 1; arg_t = &argv[i]; }
				"-h" => { show_usage(&argv[0]); },
				_    => { show_usage(&argv[0]); },
			}
			i += 1;
		}

		match ( *arg_c ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[0] ),
		}

		match ( *arg_t ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[0] ),
		}

		Options {
			input    : arg_i.to_string(),
			output   : arg_o.to_string(),
			color    : arg_c.to_string(),
			tolerate : arg_t.to_string(),
		}
	}

	pub fn show_parameter( &self )
	{
		println!( "\nParameter set:" );
		println!( "===========================================" );
		println!( "Input filename  : {}", self.input );
		println!( "Output filename : {}", self.output );
		println!( "Colorize        : {}", self.color );
		println!( "Tolerate OUBZJX : {}", self.tolerate );
		println!( "===========================================" );
	}
}

fn show_usage( arg : &String )
{
	println!( "Usage: {} [Options] \n\nOptions\n\n", *arg );
	println!( "   -i\tInput filename in aligned Multi-FASTA format, REQUIRED." );
	println!( "   -o\tOutput filename, REQUIRED." );
	println!( "   -c\tColorize each AA displayed on a terminal depending on their\n     \tstereochemical properties ('yes' or 'no', default 'no')." );
	println!( "   -t\tTolerate AA types 'O', 'U', 'B', 'Z', 'J' and 'X' in input file ('yes' or 'no', default 'yes').\n    \tIf 'no', program halts when the input file includes O, U, B, Z, J, or X." );
	println!( "   -h\tPrint this help, ignore all other arguments." );
	println!( "\n" );

	process::exit(1);
}
