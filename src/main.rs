/*
 * Project: linear-regression
 * File: main.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::*;

fn main() {
	
	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 2 {
		println!("Too mutch arguments");
		return;
	}

	let filename = &args[1];

	assert!(filename.ends_with(".csv"));

	let f = match File::open(filename) {
		Ok(f) => f,
		Err(_) => {
			println!("Impossible to open file {}", filename); 
			return
		},
	};

	let f = BufReader::new(f);

	for line in f.lines() {
		let line = match line {
			Ok(line) => line,
			Err(_) => {
				println!("Impossible read line"); 
				return
			},
		};
		println!("Line: {}", line);
	}
}
