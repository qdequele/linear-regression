/*
 * Project: linear-regression
 * File: src/resolver.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Result;

static LEARN_RATE	: f64 = 0.000_1;
static NORMALISATION: f64 = 1_000.0;
static PRECISION	: f64 = 0.000_001;

pub struct SetValue {
	pub x: f64,
	pub y: f64
}

pub struct Resolver {
	theta0 : f64, // Ø0
	theta1 : f64, // Ø1
	set : Vec<SetValue>,
	set_len : f64, // m
}

impl Resolver {

	#[no_mangle]
	pub extern fn new(set : Vec<SetValue>) -> Resolver {
		let mut resolver: Resolver = Resolver {
			theta0: 0_f64,
			theta1: 0_f64,
			set: set,
			set_len: 0_f64
		};

		resolver.set_len = resolver.set.len() as f64;
		resolver.learn();
		resolver
	}

	#[no_mangle]
	#[allow(dead_code)]
	pub fn new_from_csv(filename: &str) -> Result<Resolver> {

		assert!(filename.ends_with(".csv"));

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
		return Ok(Resolver::new(_set))
	}

	#[no_mangle]
	pub extern fn hypothesis(&self, x: f64) -> f64 {
		self.theta0 + (self.theta1 * x)
	}

	fn learn(&mut self) {

		self.set_len = self.set.len() as f64;
		let mut new_set: Vec<SetValue> = Vec::new();

		for val in &self.set {
			let _val: SetValue = SetValue {
				x: val.x / NORMALISATION,
				y: val.y / NORMALISATION
			};
			new_set.push(_val);
		}

		self.set = new_set;

		self.training_loop();
	}

	fn train(&self, set: &Vec<SetValue>) -> (f64, f64) {
		let mut sum_0 = 0_f64;
		let mut sum_1 = 0_f64;

		for val in set {
			let d = self.hypothesis(val.x) - val.y;
			sum_0 += d;
			sum_1 += d * val.x;
		}
		(LEARN_RATE * ( sum_0 / self.set_len ) , LEARN_RATE * ( sum_1 / self.set_len))
	}

	fn training_loop(&mut self) {
		loop {
			let (tmp_theta0, tmp_theta1) = self.train(&self.set);
			if tmp_theta0.abs() < PRECISION && tmp_theta1.abs() < PRECISION {
				self.theta0 = self.theta0 * NORMALISATION;
				break
			}
			self.theta0 -= tmp_theta0;
			self.theta1 -= tmp_theta1;
		}
	}

}
