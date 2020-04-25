//! Simple compile time random generator
//!
//! ## Example
//!
//! ```rust
//! use get_random_const::random;
//!
//!
//! {
//! #[random]
//! pub const RAND: usize = 0;
//!
//! #[random]
//! static STATIC_ARRAY: [usize; 5] = [];
//!
//! #[random]
//! static STATIC_ARRAY_SIGN: [isize; 5] = [];
//!
//! #[random]
//! pub const RAND_SIGN: isize = 0;
//! }
//!
//! {
//! #[random]
//! static STATIC_ARRAY: [u16; 10] = [];
//! }
//! {
//! #[random]
//! static STATIC_ARRAY: [i16; 10] = [];
//! }
//!
//! {
//! #[random]
//! static STATIC_ARRAY: [u64; 12] = [];
//! }
//! {
//! #[random]
//! static STATIC_ARRAY: [i64; 12] = [];
//! }
//!
//! #[random]
//! static STATIC_LOLKA: i128 = 0;
//! assert_ne!(STATIC_LOLKA, 0);
//!
//! #[random]
//! const LOLKA: u32 = 0;
//! assert_ne!(LOLKA, 0);
//!
//! #[random]
//! const LOLKA_ARRAY: [u32; 32] = 0;
//!
//! //Requires #![feature(proc_macro_hygiene)]
//! //#[random]
//! //let lolka: u8;
//! //assert_ne!(lolka, 0);
//!
//! ```

extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use getrandom::getrandom;

fn randomize<T: Copy>() -> T {
    let mut val = core::mem::MaybeUninit::<T>::uninit();
    let slice = unsafe {
        core::slice::from_raw_parts_mut(val.as_mut_ptr() as *mut u8, core::mem::size_of::<T>())
    };

    getrandom(slice).expect("To generate random");
    unsafe {
        val.assume_init()
    }
}

fn map_array_init(path: &syn::TypeArray) -> Option<proc_macro2::TokenStream> {
    let ty = match &*path.elem {
        &syn::Type::Path(ref path) => &path.path,
        _ => return None,
    };

    let gen: Box<dyn Fn() -> String> = if ty.is_ident("u8"){
        Box::new(|| format!("{}u8", randomize::<u8>()))
    } else if ty.is_ident("i8") {
        Box::new(|| format!("{}i8", randomize::<i8>()))
    } else if ty.is_ident("u16") {
        Box::new(|| format!("{}u16", randomize::<u16>()))
    } else if ty.is_ident("i16") {
        Box::new(|| format!("{}i16", randomize::<i16>()))
    } else if ty.is_ident("u32") {
        Box::new(|| format!("{}u32", randomize::<u32>()))
    } else if ty.is_ident("i32") {
        Box::new(|| format!("{}i32", randomize::<i32>()))
    } else if ty.is_ident("u64") {
        Box::new(|| format!("{}u64", randomize::<u64>()))
    } else if ty.is_ident("i64") {
        Box::new(|| format!("{}i64", randomize::<i64>()))
    } else if ty.is_ident("u128") {
        Box::new(|| format!("{}u128", randomize::<u128>()))
    } else if ty.is_ident("i128") {
        Box::new(|| format!("{}i128", randomize::<i128>()))
    } else if ty.is_ident("usize") {
        Box::new(|| format!("{} as usize", randomize::<usize>()))
    } else if ty.is_ident("isize") {
        Box::new(|| format!("{} as isize", randomize::<isize>()))
    } else {
        return None
    };

    let len = match &path.len {
        &syn::Expr::Lit(ref literal) => match &literal.lit {
            &syn::Lit::Int(ref int) => match int.base10_parse::<usize>() {
                Ok(result) => result,
                Err(_) => return None,
            },
            _ => return None,
        },
        _ => return None,
    };

    let mut init = "[".to_owned();

    for _ in 0..len {
        init.push_str(gen().as_str());
        init.push(',');
    }

    init.pop();
    init.push(']');

    init.parse().ok()
}

fn map_path_to_init_expr(path: &syn::Path) -> Option<proc_macro2::TokenStream> {
    let res = if path.is_ident("u8"){
        let res = randomize::<u8>();
        quote! { #res }
    } else if path.is_ident("i8") {
        let res = randomize::<i8>();
        quote! { #res }
    } else if path.is_ident("u16") {
        let res = randomize::<u16>();
        quote! { #res }
    } else if path.is_ident("i16") {
        let res = randomize::<i16>();
        quote! { #res }
    } else if path.is_ident("u32") {
        let res = randomize::<u32>();
        quote! { #res }
    } else if path.is_ident("i32") {
        let res = randomize::<i32>();
        quote! { #res }
    } else if path.is_ident("u64") {
        let res = randomize::<u64>();
        quote! { #res }
    } else if path.is_ident("i64") {
        let res = randomize::<i64>();
        quote! { #res }
    } else if path.is_ident("u128") {
        let res = randomize::<u128>();
        quote! { #res }
    } else if path.is_ident("i128") {
        let res = randomize::<i128>();
        quote! { #res }
    } else if path.is_ident("usize") {
        let res = randomize::<usize>();
        quote! { #res as usize }
    } else if path.is_ident("isize") {
        let res = randomize::<isize>();
        quote! { #res as isize }
    } else {
        return None
    };

    Some(res)
}

#[proc_macro_attribute]
pub fn random(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::Stmt);

    match &mut input {
        syn::Stmt::Local(ref mut local) => {
            let init = match &local.pat {
                syn::Pat::Type(ref typ) => match &*typ.ty {
                    syn::Type::Path(ref pt) => match map_path_to_init_expr(&pt.path) {
                        Some(init) => init,
                        None => return syn::Error::new_spanned(input, "Specified type is not simple integer").to_compile_error().into(),
                    },
                    syn::Type::Array(ref arr) => match map_array_init(arr) {
                        Some(init) => init,
                        None => return syn::Error::new_spanned(input, "Specified type is not integer array").to_compile_error().into(),
                    },
                    _ => return syn::Error::new_spanned(input, "Invalid type, must integer").to_compile_error().into(),
                },
                _ => return syn::Error::new_spanned(input, "Missing type specifier").to_compile_error().into(),
            };

            let eq = quote! {
                =
            };
            let init = quote! {
                #init
            };
            let eq = syn::parse2::<syn::token::Eq>(eq).unwrap();
            let init = syn::parse2::<syn::Expr>(init).unwrap();
            local.init = Some((eq, Box::new(init)));
        },
        syn::Stmt::Item(ref mut item) => match item {
            syn::Item::Const(ref mut item) => {
                let init = match &*item.ty {
                    syn::Type::Path(ref pt) => match map_path_to_init_expr(&pt.path) {
                        Some(init) => init,
                        None => return syn::Error::new_spanned(input, "Specified type is not simple integer").to_compile_error().into(),
                    },
                    syn::Type::Array(ref arr) => match map_array_init(arr) {
                        Some(init) => init,
                        None => return syn::Error::new_spanned(input, "Specified type is not integer array").to_compile_error().into(),
                    },
                    _ => return syn::Error::new_spanned(input, "Invalid type, must integer").to_compile_error().into(),
                };

                let init = quote! {
                    #init
                };
                let init = syn::parse2::<syn::Expr>(init).unwrap();

                item.expr = Box::new(init);
            },
            syn::Item::Static(ref mut item) => {
                let init = match &*item.ty {
                    syn::Type::Path(ref pt) => match map_path_to_init_expr(&pt.path) {
                        Some(init) => init,
                        None => return syn::Error::new_spanned(input, "Specified type is not simple integer").to_compile_error().into(),
                    },
                    syn::Type::Array(ref arr) => match map_array_init(arr) {
                        Some(init) => init,
                        None => return syn::Error::new_spanned(input, "Specified type is not integer array").to_compile_error().into(),
                    },
                    _ => return syn::Error::new_spanned(input, "Invalid type, must integer").to_compile_error().into(),
                };

                let init = quote! {
                    #init
                };
                let init = syn::parse2::<syn::Expr>(init).unwrap();

                item.expr = Box::new(init);
            },
            _ => return syn::Error::new_spanned(input, "Unsupported item").to_compile_error().into(),
        },
        _ => return syn::Error::new_spanned(input, "Unsupported statement").to_compile_error().into(),
    }

    let res = quote! {
        #input
    };

    res.into()
}
