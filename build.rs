fn main() {
	// currently, we only support external primesieve
    println!("cargo:rustc-link-lib=primesieve");
}

