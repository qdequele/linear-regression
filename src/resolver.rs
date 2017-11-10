/*
 * Project: linear-regression
 * File: src/resolver.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

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

	#[allow(dead_code)]
	pub fn learn(&mut self) {

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

	#[allow(dead_code)]
	pub fn new(set : Vec<SetValue>) -> Resolver {
		let mut resolver: Resolver = Resolver {
			theta0: 0_f64,
			theta1: 0_f64,
			set		: set,
			set_len: 0_f64
		};

		resolver.set_len = resolver.set.len() as f64;
		resolver
	}

	pub fn hypothesis(&self, x: f64) -> f64 {
		self.theta0 + (self.theta1 * x)
	}
	
	fn train(&self, set: &Vec<SetValue>) -> (f64, f64) {
		let m = self.set_len;
		let mut sum_0 = 0_f64;
		let mut sum_1 = 0_f64;

		for val in set {
			let d = self.hypothesis(val.x) - val.y;
			sum_0 += d;
			sum_1 += d * val.x;
		}
		(LEARN_RATE * ( sum_0 / m ) , LEARN_RATE * ( sum_1 / m))
	}

	fn training_loop(&mut self) {
		let mut i = 0_u32;

		loop {
			let (tmp_theta0, tmp_theta1) = self.train(&self.set);
			if tmp_theta0.abs() < PRECISION && tmp_theta1.abs() < PRECISION {
				self.theta0 = self.theta0 * NORMALISATION;
				break
			}
			self.theta0 -= tmp_theta0;
			self.theta1 -= tmp_theta1;
			i+=1;
		}
		println!("precision : {}", i);
	}

}
