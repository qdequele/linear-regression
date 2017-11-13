/*
 * Project: linear-regression
 * File: src/estimate.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::string::*;

static DATA_FILE	: &'static str = ".data";


fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 2 {
		println!("Too mutch arguments");
		return;
	}

	if args[1].parse::<f64>().is_ok() {
		let res = get_price(args[1].parse().unwrap());
		if res < 0_f64 {
			println!("Too many kilometers, this kind of cars could not exist");
		} else {
			println!("For {} km the price is {}", args[1],res);
		}
	} else {
		println!("{:?} is not a number", &args[1]);
	}
}

pub extern fn get_price(x: f64) -> f64 {
	let (theta0, theta1) = get_data();
	theta0 + (theta1 * x)
}

fn get_data() -> (f64, f64) {
	let path = Path::new(DATA_FILE);

	let mut file = match File::open(&path) {
		Err(_) => {
			return (0_f64, 0_f64)
		},
		Ok(file) => file,
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(_) => {
			return (0_f64, 0_f64)
		},
		Ok(_) => {},
	}

	let tab : Vec<&str> = s.split(";").collect();

	if tab[0].parse::<f64>().is_ok() && tab[1].parse::<f64>().is_ok() {
		return (tab[0].parse().unwrap(), tab[1].parse().unwrap())
	}

	return (0_f64, 0_f64)
}
