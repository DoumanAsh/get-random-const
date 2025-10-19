# get-random-const

[![Rust](https://github.com/DoumanAsh/get-random-const/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/get-random-const/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/get-random-const.svg)](https://crates.io/crates/get-random-const)
[![Documentation](https://docs.rs/get-random-const/badge.svg)](https://docs.rs/crate/get-random-const/)

Macro to generate random at compile time

```rust
use get_random_const::random;

const RANDOM_U8: u8 = random!(u8);
assert_ne!(RANDOM_U8, 0);

static RANDOM_I32: i32 = random!(i32);
assert_ne!(RANDOM_I32, 0);

let random_u32 = random!(u32);
assert_ne!(random_u32, 0);

let random_array = random!([u32;5]);
assert_eq!(random_array.len(), 5);

for elem in random_array.iter() {
    assert_ne!(*elem, 0);
}

let random_array: [u32; 0] = random!([u32;0]); //Well, I guess you can if you want?
assert_eq!(random_array.len(), 0);
```
