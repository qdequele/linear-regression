/*
 * Project: linear-regression
 * File: src/estimate.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

static DATA_FILE	: &'static str = ".data";

pub extern fn get_price(x: f64) -> f64 {
	let (theta0, theta1) = get_data();
	theta0 + (theta1 * x)
}

fn get_data() -> (f64, f64) {
	let path = Path::new(DATA_FILE);
	let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => {
			println!("couldn't open {}: {}",
				display,
				why.description());
			return (0_f64, 0_f64)
		},
		Ok(file) => file,
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(why) => {
			println!("couldn't read {}: {}",
				display,
				why.description());
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
