/*
 * Project: linear-regression
 * File: main.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */


use std::env;
use std::string::*;

mod estimate;
mod learn;

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 2 {
		println!("Too mutch arguments");
		return;
	}

	learn::learn(&args[1]);

	let res_1 = estimate::get_price(80_000_f64);
	println!("res for 80_000 : {}", res_1);
	let res_2 = estimate::get_price(120_000_f64);
	println!("res for 120_000 : {}", res_2);
	let res_3 = estimate::get_price(48_235_f64);
	println!("res for 48_235 : {}", res_3);
}


