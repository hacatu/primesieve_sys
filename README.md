# `primesieve_sys`

`primesieve_sys` is a Rust wrapper for [Primesieve](https://github.com/kimwalisch/primesieve).

## Installation

Currently, you must have `primesieve` installed on your system.
If it is not in a standard location, you may need to edit `Cargo.toml` to include `"-Clink-arg=-L/usr/local/lib"` in `rustflags`.

## Usage
Add `primesieve_sys` to your `Cargo.toml`:

```toml
[dependencies]
primesieve_sys = "0.1.0" # Replace with the latest version from crates.io
```

Then, in your Rust code:

```rust
use primesieve_sys::generate_primes;

fn main() {
	let primes = generate_primes(1_000_000_0000u64);
	let primes: &[_] = primes.borrow();
	let count = primes.iter().filter(|&&p|p&4==1).count();
	println!("Found {count} Pythagorean primes up to 10^9");
}
```

## Links
- **Crate**: [https://crates.io/crates/primesieve_sys](https://crates.io/crates/primesieve_sys)
- **Documentation**: [https://docs.rs/primesieve_sys](https://docs.rs/primesieve_sys)  

## License
`primesieve_sys` is licensed under [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/legalcode.en). See `LICENSE` for details.

