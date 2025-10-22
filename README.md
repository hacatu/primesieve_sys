# `primesieve_wrapper`

`primesieve_wrapper` is a Rust wrapper for [Primesieve](https://github.com/kimwalisch/primesieve).

## Installation

Currently, you must have `primesieve` installed on your system.
If it is not in a standard location, you may need to edit `Cargo.toml` to include `"-Clink-arg=-L/usr/local/lib"` in `rustflags`.

## Usage
Add `primesieve_wrapper` to your `Cargo.toml`:

```toml
[dependencies]
primesieve_wrapper = "0.1.0" # Replace with the latest version from crates.io
```

Then, in your Rust code:

```rust
use primesieve_wrapper::generate_primes;

fn main() {
	let primes = generate_primes(1_000_000_0000u64);
	let primes: &[_] = primes.borrow();
	let count = primes.iter().filter(|&&p|p&4==1).count();
	println!("Found {count} Pythagorean primes up to 10^9");
}
```

## Links
- **Crate**: [https://crates.io/crates/primesieve_wrapper](https://crates.io/crates/primesieve_wrapper)
- **Documentation**: [https://docs.rs/primesieve_wrapper](https://docs.rs/primesieve_wrapper)  

## License
`primesieve_wrapper` is licensed under [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/legalcode.en). See `LICENSE` for details.
Primesieve is licensed under [BSD-2-Clause](https://opensource.org/license/bsd-2-clause)

