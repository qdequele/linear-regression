/*
 * Project: linear-regression
 * File: main.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */


use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::*;

mod resolver;

use resolver::{SetValue};

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
	let mut _set : Vec<SetValue> = Vec::new();

	for line in f.lines() {
		let line = match line {
			Ok(line) => line,
			Err(_) => {
				println!("Impossible read line"); 
				return
			},
		};
		let tab : Vec<&str> = line.split(",").collect();
		let _x : &str = tab[0];
		let _y : &str = tab[1];

		if _x.parse::<f64>().is_ok() && _y.parse::<f64>().is_ok() {
			let value : SetValue = SetValue {
				x: _x.parse().unwrap(),
				y: _y.parse().unwrap()
			};
			_set.push(value);
		}
	}

	let mut resolver = resolver::Resolver::new(_set);
	resolver.learn();

	let res_1 = resolver.hypothesis(80_000_f64);
	println!("res for 80_000 : {}", res_1);
	let res_2 = resolver.hypothesis(120_000_f64);
	println!("res for 120_000 : {}", res_2);
}
