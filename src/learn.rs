/*
 * Project: linear-regression
 * File: src/learn.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Result;
use std::io::Error as ioError;
use std::io::ErrorKind;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

static LEARN_RATE	: f64 = 0.000_1;
static NORMALISATION: f64 = 1_000.0;
static PRECISION	: f64 = 0.000_001;
static DATA_FILE	: &'static str = ".data";

pub struct SetValue {
	pub x: f64,
	pub y: f64
}

pub struct Theta {
	pub _0: f64,
	pub _1: f64
}

fn hypothesis(theta: &Theta, x: f64) -> f64 {
	theta._0 + (theta._1 * x)
}

pub fn learn(filename: &str) {
	let mut set = match get_csv(filename) {
		Ok(set) => set,
		Err(e) => {
			println!("{:?}", e);
			return
		}
	};

	let mut new_set: Vec<SetValue> = Vec::new();

	for val in &set {
		let _val: SetValue = SetValue {
			x: val.x / NORMALISATION,
			y: val.y / NORMALISATION
		};
		new_set.push(_val);
	}

	set = new_set;

	let theta = training_loop(&set);
	save_data(theta);
}

fn train(set: &Vec<SetValue>, theta: &Theta) -> Theta {

	let mut sum_0 = 0_f64;
	let mut sum_1 = 0_f64;
	let set_len = set.len() as f64;

	for val in set {
		let d = hypothesis(&theta, val.x) - val.y;
		sum_0 += d;
		sum_1 += d * val.x;
	}
	
	Theta {
		_0: LEARN_RATE * ( sum_0 / set_len ), 
		_1: LEARN_RATE * ( sum_1 / set_len)
	}
}

fn training_loop(set: &Vec<SetValue>) -> Theta {
	let mut theta : Theta = get_data();
	
	loop {
		let tmp_theta: Theta = train(&set, &theta);
		if tmp_theta._0.abs() < PRECISION && tmp_theta._1.abs() < PRECISION {
			theta._0 = theta._0 * NORMALISATION;
			break
		}
		theta._0 -= tmp_theta._0;
		theta._1 -= tmp_theta._1;
	}

	theta
}

fn save_data(theta: Theta) {
	let path = Path::new(DATA_FILE);
	let display = path.display();

	let mut file = match File::create(&path) {
		Err(why) => {
			println!("couldn't create {}: {}",
				display,
				why.description());
			return
		},
		Ok(file) => file,
	};

	let data = format!("{};{}", theta._0, theta._1);
	match file.write_all(data.as_bytes()) {
		Err(why) => {
			println!("couldn't write to {}: {}", 
				display,
				why.description());
			return
		},
		Ok(_) => {},
	}
}

fn get_csv(filename: &str) -> Result<Vec<SetValue>> {

	if !filename.ends_with(".csv") {
		return Err(ioError::new(ErrorKind::Other, "file is not a csv"))
	};

	let f = match File::open(filename) {
		Ok(f) => f,
		Err(e) => return Err(e)
	};

	let f = BufReader::new(f);
	let mut _set : Vec<SetValue> = Vec::new();

	for line in f.lines() {
		let line = match line {
			Ok(line) => line,
			Err(e) => return Err(e)
		};

		let tab : Vec<&str> = line.split(",").collect();

		if tab[0].parse::<f64>().is_ok() && tab[1].parse::<f64>().is_ok() {
			_set.push(SetValue {
				x: tab[0].parse().unwrap(),
				y: tab[1].parse().unwrap()
			});
		}
	}
	return Ok(_set)
}

fn get_data() -> Theta {
	let path = Path::new(DATA_FILE);
	let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => {
			println!("couldn't open {}: {}",
				display,
				why.description());
			return Theta {
				_0: 0_f64,
				_1: 0_f64
			}
		},
		Ok(file) => file,
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(why) => {
			println!("couldn't read {}: {}",
				display,
				why.description());
			return Theta {
				_0: 0_f64,
				_1: 0_f64
			}
		},
		Ok(_) => {},
	}

	let tab : Vec<&str> = s.split(";").collect();

	if tab[0].parse::<f64>().is_ok() && tab[1].parse::<f64>().is_ok() {
		return Theta {
			_0: tab[0].parse().unwrap(),
			_1: tab[1].parse().unwrap()
		}
	}

	Theta {
		_0: 0_f64,
		_1: 0_f64
	}
}
