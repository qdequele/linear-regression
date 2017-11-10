/*
 * Project: linear-regression
 * File: main.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */


use std::env;
use std::string::*;

mod resolver;

use resolver::{Resolver};

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 2 {
		println!("Too mutch arguments");
		return;
	}

	let resolver = match Resolver::new_from_csv(&args[1]) {
		Ok(resolver) => resolver,
		Err(e) => {
			println!("{:?}", e);
			return
		}
	};

	let res_1 = resolver.hypothesis(80_000_f64);
	println!("res for 80_000 : {}", res_1);
	let res_2 = resolver.hypothesis(120_000_f64);
	println!("res for 120_000 : {}", res_2);
}


