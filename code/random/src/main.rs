// extern crate rand_core;
// use rand_core;
// use rand_core::{RngCore, Error, impls};

extern crate rand;
use rand::{Rng, thread_rng};

fn test0() {

	// let _g = rand_core::RngCore();
}

fn test1() {
	// let mut rng = rand::thread_rng();
	let n: i64 = thread_rng().gen();
    println!("{}", n);
}

fn main() {

	test0();
	test1();
}
