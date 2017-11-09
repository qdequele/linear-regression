/*
 * Project: linear-regression
 * File: src/resolver.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

pub struct SetValue {
	x: f64,
	y: f64,
}

pub struct Resolver {
	// theta0: f64, // Ø0
	// theta1: f64, // Ø1
	delta_theta: f64, // DØ = 1000.0
	learning_rate: f64, // alpha = 2.0
	number_of_training: u64, // m = 100
	// tmp_theta0: f64,
	// tmp_theta1: f64,
	set: Vec<SetValue>,
}

impl Resolver {

	#[allow(dead_code)]
	pub fn learn(&mut self) {
		self.calculate_gradient_descent_minimization();
	}

	#[allow(dead_code)]
	pub fn new() -> Resolver {
		Resolver {
			delta_theta: 3.0,
			learning_rate: 0.1,
			number_of_training: 1_000
		}
	}

	pub fn hypothesis(&mut self, x: f64) -> f64 {
		return self.theta0 + (self.theta1 * x)
	}

	fn train(&mut self) -> (f64, f64) {

		let m = self.set.len() as f64;
		let mut sum_0 = 0_f64;
		let mut sum_1 = 0_f64;

		for val in self.set {
			let d = self.hypothesis(val.x) * val.y;
			sum_0 += d;
			sum_1 += d * val.x;
		}

		(self.learning_rate * ( 1_f64 / m) * sum_0, self.learning_rate * ( 1_f64 / m) * sum_1);
	}

	fn training_loop(&mut self) {
		for _ in 0..self.number_of_training {
			let (tmp_theta0, tmp_theta1) = self.train();
			self.theta0 -= tmp_theta0;
			self.theta1 -= tmp_theta1;
		}
	}

	fn calculate_gradient_descent_minimization(&mut self) {
		for x in 0..self.number_of_training {
			println!("{} \t| {} \t| {}", x, self.delta_theta, (self.learning_rate * 2.0 * self.delta_theta));
			self.delta_theta = self.delta_theta - (self.learning_rate * 2.0 * self.delta_theta)
		}
		println!("result theta = {}", self.delta_theta);
	}
}
