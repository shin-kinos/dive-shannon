
use std::collections::HashMap;
use std::f64;

static mut SYMBOL : Vec<char> = Vec::new();

pub fn get_shan_ent( site_list : &Vec<String>, arg_t : &String ) -> Vec<f64>
{
	unsafe {
		if *arg_t == "no" { SYMBOL = "ARNDCQEGHILKMFPSTWYV-".chars().collect(); }
		else              { SYMBOL = "ARNDCQEGHILKMFPSTWYVBZXU-".chars().collect(); }
	}

	let mut shan_ent_list : Vec<f64> = Vec::new();
	for site in ( *site_list ).iter() {
		shan_ent_list.push( cal_shan_ent(site) );
	}

	shan_ent_list
}

fn cal_shan_ent( arg : &String ) -> f64
{
	let site : Vec<char> = ( *arg ).chars().collect();
	//println!( "site : {:?}", site );

	let mut freq : HashMap<char, usize> = HashMap::new();

	/*
	let mut _symbol : Vec<char> = Vec::new();
	if *arg_ig == "no" { _symbol = "ARNDCQEGHILKMFPSTWYV-".chars().collect(); }
	else               { _symbol = "ARNDCQEGHILKMFPSTWYVBZXU-".chars().collect(); }
	*/


	/*
	let _symbol : [char; 24] = [
		'A',
		'R',
		'N',
		'D',
		'C',
		'Q',
		'E',
		'G',
		'H',
		'I',
		'L',
		'K',
		'M',
		'F',
		'P',
		'S',
		'T',
		'W',
		'Y',
		'V',
		'B',
		'Z',
		'X',
		'-'
	];
	*/

	/*
	for aa in _symbol.iter() { freq.insert(*aa, 0); }
	//println!( "{:?}", freq );
	*/

	unsafe {
		for aa in SYMBOL.iter() { freq.insert(*aa, 0); }
	}
	//println!( "{:?}", freq );

	for aa in site.iter() {
		let inc : usize = freq[ aa ] + 1;
		freq.insert( *aa, inc );
	}
	//println!( "{:?}", freq );

	let n : usize = site.len();

	let mut shan_ent : f64 = 0.0;

	/*
	for aa in _symbol.iter() {
		if freq[ aa ] != 0 {
			//println!( "Probability of {} : {}", *aa, (freq[ aa ] as f64) / (n as f64) );
			let prob : f64 = ( freq[ aa ] as f64 ) / ( n as f64 );
			shan_ent += prob * prob.log2();
		}
	}
	*/

	unsafe {
		for aa in SYMBOL.iter() {
			if freq[ aa ] != 0 {
				//println!( "Probability of {} : {}", *aa, (freq[ aa ] as f64) / (n as f64) );
				let prob : f64 = ( freq[ aa ] as f64 ) / ( n as f64 );
				shan_ent += prob * prob.log2();
			}
		}
	}

	shan_ent = -1.0 * shan_ent;

	shan_ent = ( shan_ent * 1000.0 ).round() / 1000.0;

	//println!( "\nShannon entropy : {}", shan_ent );

	shan_ent
}
