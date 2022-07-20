use std::ops::{Range, RangeInclusive};

use std::process::Command;
// use rand::prelude::*;
#[cfg(feature = "array")]use rhai::Array;
#[cfg(feature = "float")]use rhai::FLOAT;
use rhai::{def_package, plugin::*, EvalAltResult, Position, INT};

def_package! {
		/// Package for random number generation, sampling and shuffling.
		pub CmdPackage(lib) {
				// combine_with_exported_module!(lib, "rand", rand_functions);

				// #[cfg(feature = "array")]
				// combine_with_exported_module!(lib, "array", array_functions);
		}
}

#[export_module]
mod cmd_functions {
	/// Spawn a system process, returning an object map, `Output`, wrapping `status,Std{In,Out,Err}`.
	/// Spawn a system process, returning an object map, `Child`, wrapping `Std{In,Out,Err}`.
	///
	/// # Example
	///
	/// ```rhai
	/// let ls_output = cmd("whoami");
  /// 
	/// ```
	pub fn cmd(cmd: &str) -> RhaiChild { Command::new(cmd).output() }

	/// Generate a random boolean value with a probability of being `true`.
	///
	/// `probability` must be between `0.0` and `1.0` (inclusive).
	///
	/// # Example
	///
	/// ```rhai
	/// let decision = rand(0.01);  // 1% probability
	///
	/// if decision {
	///     print("You hit the Jackpot!")
	/// }
	/// ```
	#[cfg(feature = "float")]
	#[rhai_fn(name = "rand", return_raw)]
	pub fn rand_bool_with_probability(probability: FLOAT) -> Result<bool, Box<EvalAltResult>> {
		if probability < 0.0 || probability > 1.0 {
			Err(
				EvalAltResult::ErrorArithmetic(
					format!("Invalid probability (must be between 0.0 and 1.0): {}", probability),
					Position::NONE,
				)
				.into(),
			)
		} else {
			Ok(rand::thread_rng().gen_bool(probability as f64))
		}
	}

	/// Generate a random integer number.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand();
	///
	/// print(`I'll give you a random number: ${number}`);
	/// ```
	pub fn rand() -> INT { rand::random() }

	/// Generate a random integer number within an exclusive range.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand(18..39);
	///
	/// print(`I'll give you a random number between 18 and 38: ${number}`);
	/// ```
	#[rhai_fn(name = "rand", return_raw)]
	pub fn rand_exclusive_range(range: Range<INT>) -> Result<INT, Box<EvalAltResult>> {
		if range.is_empty() {
			Err(
				EvalAltResult::ErrorArithmetic(format!("Range is empty: {:?}", range), Position::NONE)
					.into(),
			)
		} else {
			Ok(rand::thread_rng().gen_range(range))
		}
	}

	/// Generate a random integer number within an inclusive range.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand(18..=38);
	///
	/// print(`I'll give you a random number between 18 and 38: ${number}`);
	/// ```
	#[rhai_fn(name = "rand", return_raw)]
	pub fn rand_inclusive_range(range: RangeInclusive<INT>) -> Result<INT, Box<EvalAltResult>> {
		if range.is_empty() {
			Err(
				EvalAltResult::ErrorArithmetic(format!("Range is empty: {:?}", range), Position::NONE)
					.into(),
			)
		} else {
			Ok(rand::thread_rng().gen_range(range))
		}
	}

	/// Generate a random integer number within an inclusive range.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand(18, 38);
	///
	/// print(`I'll give you a random number between 18 and 38: ${number}`);
	/// ```
	#[rhai_fn(name = "rand", return_raw)]
	pub fn rand_from_to_inclusive(start: INT, end: INT) -> Result<INT, Box<EvalAltResult>> {
		if start >= end {
			Err(
				EvalAltResult::ErrorArithmetic(
					format!("Range is empty: {}..{}", start, end),
					Position::NONE,
				)
				.into(),
			)
		} else {
			Ok(rand::thread_rng().gen_range(start..=end))
		}
	}

	/// Generate a random floating-point number between `0.0` and `1.0` (exclusive).
	///
	/// `1.0` is _excluded_ from the possibilities.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand_float();
	///
	/// print(`I'll give you a random number between 0 and 1: ${number}`);
	/// ```
	#[cfg(feature = "float")]
	pub fn rand_float() -> FLOAT { rand::random() }
	/// Generate a random floating-point number within an exclusive range.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand_float(123.456, 789.678);
	///
	/// print(`I'll give you a random number between 123.456 and 789.678: ${number}`);
	/// ```
	#[cfg(feature = "float")]
	#[rhai_fn(name = "rand_float", return_raw)]
	pub fn rand_float_range(start: FLOAT, end: FLOAT) -> Result<FLOAT, Box<EvalAltResult>> {
		if start >= end {
			Err(
				EvalAltResult::ErrorArithmetic(
					format!("Range is empty: {}..{}", start, end),
					Position::NONE,
				)
				.into(),
			)
		} else {
			Ok(rand::thread_rng().gen_range(start..=end))
		}
	}

	/// Generate a random [decimal](https://crates.io/crates/rust_decimal) number.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand_decimal();
	///
	/// print(`I'll give you a random decimal number: ${number}`);
	/// ```
	#[cfg(feature = "decimal")]
	pub fn rand_decimal() -> rust_decimal::Decimal { rand::random() }
	/// Generate a random [decimal](https://crates.io/crates/rust_decimal) number within a range.
	///
	/// # Example
	///
	/// ```rhai
	/// let number = rand(18.to_decimal(), 38.to_decimal());
	///
	/// print(`I'll give you a random number between 18 and 38: ${number}`);
	/// ```
	#[cfg(feature = "decimal")]
	#[rhai_fn(name = "rand_decimal", return_raw)]
	pub fn rand_decimal_range(
		start: rust_decimal::Decimal,
		end: rust_decimal::Decimal,
	) -> Result<rust_decimal::Decimal, Box<EvalAltResult>> {
		if start >= end {
			Err(
				EvalAltResult::ErrorArithmetic(
					format!("Range is empty: {}..{}", start, end),
					Position::NONE,
				)
				.into(),
			)
		} else {
			Ok(rand::thread_rng().gen_range(start..=end))
		}
	}
}

#[cfg(feature = "array")]
#[export_module]
mod array_functions {
	/// Copy a random element from the array and return it.
	///
	/// # Example
	///
	/// ```rhai
	/// let x = [1, 2, 3, 4, 5];
	///
	/// let number = x.sample();
	///
	/// print(`I'll give you a random number between 1 and 5: ${number}`);
	/// ```
	#[rhai_fn(global)]
	pub fn sample(array: &mut Array) -> rhai::Dynamic {
		if !array.is_empty() {
			let mut rng = rand::thread_rng();
			if let Some(res) = array.choose(&mut rng) {
				return res.clone();
			}
		}
		Dynamic::UNIT
	}

	/// Copy a non-repeating random sample of elements from the array and return it.
	///
	/// Elements in the return array are likely not in the same order as in the original array.
	///
	/// * If `amount` ≤ 0, the empty array is returned.
	/// * If `amount` ≥ length of array, the entire array is returned, but shuffled.
	///
	/// # Example
	///
	/// ```rhai
	/// let x = [1, 2, 3, 4, 5];
	///
	/// let samples = x.sample(3);
	///
	/// print(`I'll give you 3 random numbers between 1 and 5: ${samples}`);
	/// ```
	#[rhai_fn(global, name = "sample")]
	pub fn sample_with_amount(array: &mut Array, amount: rhai::INT) -> Array {
		if array.is_empty() || amount <= 0 {
			return Array::new();
		}

		let mut rng = rand::thread_rng();
		let amount = amount as usize;

		if amount >= array.len() {
			let mut res = array.clone();
			res.shuffle(&mut rng);
			res
		} else {
			let mut res: Array = array.choose_multiple(&mut rng, amount).cloned().collect();
			// Although the elements are selected randomly, the order of elements in
			// the buffer is neither stable nor fully random. So we must shuffle the
			// result to achieve random ordering.
			res.shuffle(&mut rng);
			res
		}
	}

	/// Shuffle the elements in the array.
	///
	/// # Example
	///
	/// ```rhai
	/// let x = [1, 2, 3, 4, 5];
	///
	/// x.shuffle();    // shuffle the elements inside the array
	/// ```
	#[rhai_fn(global)]
	pub fn shuffle(array: &mut Array) {
		let mut rng = rand::thread_rng();
		array.shuffle(&mut rng);
	}
}
