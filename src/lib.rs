//! Simple compile time random generator
//!
//! ## Example
//!
//! ```rust
//! use get_random_const::random;
//!
//! const RANDOM_U8: u8 = random!(u8);
//! assert_ne!(RANDOM_U8, 0);
//!
//! static RANDOM_I32: i32 = random!(i32);
//! assert_ne!(RANDOM_I32, 0);
//!
//! let random_u32 = random!(u32);
//! assert_ne!(random_u32, 0);
//!
//! let random_array = random!([u32;5]);
//! assert_eq!(random_array.len(), 5);
//!
//! for elem in random_array.iter() {
//!     assert_ne!(*elem, 0);
//! }
//!
//! let random_array: [u32; 0] = random!([u32;0]); //Well, I guess you can if you want?
//! assert_eq!(random_array.len(), 0);
//! ```


extern crate proc_macro;

use proc_macro::TokenStream;
use getrandom::getrandom;

fn randomize<T: Copy>() -> T {
    let mut val = core::mem::MaybeUninit::<T>::uninit();
    let slice = unsafe {
        core::slice::from_raw_parts_mut(val.as_mut_ptr() as *mut u8, core::mem::size_of::<T>())
    };

    getrandom(slice).expect("Failed to generate random number");
    unsafe {
        val.assume_init()
    }
}

fn randomize_type(path: &str) -> Option<String> {
    let res = if path.eq_ignore_ascii_case("u8"){
        let res = randomize::<u8>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("i8") {
        let res = randomize::<i8>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("u16") {
        let res = randomize::<u16>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("i16") {
        let res = randomize::<i16>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("u32") {
        let res = randomize::<u32>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("i32") {
        let res = randomize::<i32>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("u64") {
        let res = randomize::<u64>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("i64") {
        let res = randomize::<i64>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("u128") {
        let res = randomize::<u128>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("i128") {
        let res = randomize::<i128>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("usize") {
        let res = randomize::<usize>();
        format!("{}", res)
    } else if path.eq_ignore_ascii_case("isize") {
        let res = randomize::<isize>();
        format!("{}", res)
    } else {
        return None
    };

    Some(res)
}

#[proc_macro]
pub fn random(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim();

    if input.is_empty() {
        panic!("Empty input :(");
    } else if input.starts_with('[') {
        if !input.ends_with(']') {
            panic!("'{}' is missing right bracket", input);
        }

        let array_content = input[1..input.len()-1].trim();
        if array_content.is_empty() {
            panic!("just empty brackets? Did you mean array?");
        }

        let mut split = array_content.splitn(2, ';');
        let typ = split.next().unwrap().trim();

        if randomize_type(typ).is_none() {
            panic!("'{}' is unsupported", typ);
        }

        let num = split.next().expect("Missing ';' in array type").trim();
        let num: usize = num.parse().expect("Array type size is invalid as usize");

        let mut result = "[".to_owned();
        if num > 0 {
            for _ in 0..num {
                result.push_str(randomize_type(typ).unwrap().as_str());
                result.push(',');
            }
            result.pop();
        }
        result.push(']');

        result.parse().unwrap()
    } else if let Some(result) = randomize_type(input) {
        return result.parse().unwrap()
    } else {
        panic!("Unsupported type");
    }
}
