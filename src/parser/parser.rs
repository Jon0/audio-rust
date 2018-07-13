use std::fs::File;
use std::io::prelude::*;


pub fn read_file(filename: &str) {

	println!("In file {}", filename);

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	let lines: Vec<&str> = contents.split('\n').collect();


	println!("With text:\n{}", contents);
}
