
use std::collections::HashMap;
use std::f64;

static mut SYMBOL : Vec<char> = Vec::new();

pub fn get_shan_ent( site_list : &Vec<String>, arg_t : &String ) -> Vec<f64>
{
	unsafe {
		if *arg_t == "no" { SYMBOL = "ARNDCQEGHILKMFPSTWYV-".chars().collect(); }
		else              { SYMBOL = "ARNDCQEGHILKMFPSTWYVOUBZJX-".chars().collect(); }
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

	//Define the pseudocount (10e-7).
	let pseudocount : f64 = 0.0000001;

	let mut freq : HashMap<char, f64> = HashMap::new();

	unsafe {
		for aa in SYMBOL.iter() { freq.insert(*aa, pseudocount); }
	}
	//println!( "{:?}", freq );

	for aa in site.iter() {
		//Count the number of the each AA in a site.
		let inc : f64 = freq[ aa ] + 1.0;
		freq.insert( *aa, inc );
	}
	//println!( "{:?}", freq );

	let n : usize = site.len();

	let mut shan_ent : f64 = 0.0;

	unsafe {
		for aa in SYMBOL.iter() {
			/*
			Calculate Shannon's entropy in a site.
			n = Denominator of the probability, length of a site.
			freq[ aa ] = Numerator of the probability, frequency of the AA in the site.
			prob = Probability.
			shan_ent = Shannon's entropy.
			*/
			let prob : f64 = freq[ aa ] / ( n as f64 );
			shan_ent += prob * prob.log2();
		}
	}

	shan_ent = -1.0 * shan_ent;

	shan_ent = ( shan_ent * 1000.0 ).round() / 1000.0;

	//println!( "\nShannon entropy : {}", shan_ent );

	shan_ent
}
