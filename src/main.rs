
mod options;
mod fasta;
mod shannon_ent;
mod result;

fn main()
{
	println!( "\nCalculate Shannon's entropy a site in MSA.\n" );

	//Command line parser.
	let opts = options::Options::new();
	opts.show_parameter();

	//Read an input file and get FASTA information.
	let data = fasta::Fasta::new( &(opts.input) );
	data.check_fasta_info();

	/*
	println!("Inputfile content");
	for i in 0 .. ( data.sequence ).len() {
		println!( "Tile {} : {}", i + 1, ( data.title )[i] );
		println!(　"Sequence {} : {}", i + 1, ( data.sequence )[i] );
	}
	*/

	//Get site information to calculate Shannon-entropy.
	let site_list : Vec<String> = data.get_site_list();
	//println!( "{:?}", site_list );

	//Calculate Shannon-entropy.
	let shan_ent_list : Vec<f64> = shannon_ent::get_shan_ent( &site_list, &(opts.tolerate) );
	//println!( "{:?}", shan_ent_list );

	//Show result.
	result::show_result( &site_list, &shan_ent_list, &(opts.color) );

	//Save result.
	result::save_result( &site_list, &shan_ent_list, &(opts.output) );

	println!( "\nProgram completed !!!\n" );

}
