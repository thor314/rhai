//! A batteries-included library template.
// TODO: remove these when ready
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::error::Error;

use anyhow::Result;
use rhai::{Dynamic, Engine, EvalAltResult, Scope};

mod error;
mod utils;

// testing out the rhai scripting language
// https://rhai.rs/book/start/features.html
// examples: https://rhai.rs/book/start/examples/scripts.html
pub fn rhai_1() -> Result<(), Box<EvalAltResult>> {
	let engine = Engine::new();
	let script = "print(40 + 2);";
	engine.run(script)?;
	// or evaluate an answer:
	let result = engine.eval::<i64>("40 + 2")?;
	println!("{result:?}");
	let result = engine.eval_file::<i64>("scripts/hello_world.rhai".into())?; // some file
	println!("{result:?}");
	// or use dynamic typing:
	let result = engine.eval_file::<Dynamic>("scripts/hello_world.rhai".into())?; // some file
	println!("{result:?}");
	Ok(())
}

// register a Rust function for use in Rhai:
pub fn to_rhai(s: &str) -> usize { s.len() }

pub fn consume_to_rhai() -> Result<(), Box<dyn std::error::Error>> {
	let mut engine = Engine::new();
	// note, if to_rhai returns Result, use register_result_fn instead
	engine.register_fn("to_rhai", to_rhai);
	let result = engine.eval::<usize>(r#"to_rhai("abcde")"#)?;
	dbg!(result);
	Ok(())
}

// call Rhai from Rust
pub fn rhai_to_rust() -> Result<(), Box<dyn std::error::Error>> {
	let engine = Engine::new();
	let ast = engine.compile_file("scripts/my_fn.rhai".into())?;
    // pass state through with Scope
	let mut scope = Scope::new();
    // generally prefer using i64's
	let my_local_var = 42i64;
	scope.push("my_local_var", my_local_var);
    // to pass in custom types, impl FuncArgs for Struct
	let result: i64 = engine.call_fn(&mut scope, &ast, "hello", ("my rhai argument", 1i64))?;
    // can also use rhai functions to create rust closures.
	println!("result: {result:?}");
	Ok(())
}



fn main() -> Result<(), Box<dyn Error>> {
	rhai_1()?;
	consume_to_rhai()?;
    rhai_to_rust()?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rhai_1() {
		// let r = rhai_1();
		// assert!(r);

		let a = 2usize;
		assert_eq!(2 + 2, 4);
	}
}
