//! Simple compile time random generator
//!
//! ## Example
//!
//! ```rust
//! #![feature(proc_macro_hygiene)]
//! use get_random_const::random;
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
//! let lolka: u8;
//! assert_ne!(lolka, 0);
//!
//! println!("lolka={}", lolka);
//!
//! //#[random]
//! //let lolka_arr: [u16; 14];
//!
//! //println!("lolka_arr={:?}", lolka_arr);
//! ```

extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use getrandom::getrandom;

fn randomize<T: Copy>(val: &mut T) {
    let slice = unsafe {
        core::slice::from_raw_parts_mut(val as *mut _ as *mut u8, core::mem::size_of::<T>())
    };

    getrandom(slice).expect("To generate random");
}

fn map_path_to_init_expr(path: &syn::Path) -> Option<proc_macro2::TokenStream> {
    let res = if path.is_ident("u8"){
        let mut res = 0u8;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("i8") {
        let mut res = 0i8;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("u16") {
        let mut res = 0i16;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("i16") {
        let mut res = 0i16;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("u32") {
        let mut res = 0u32;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("i32") {
        let mut res = 0i32;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("u64") {
        let mut res = 0u64;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("i64") {
        let mut res = 0i64;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("u128") {
        let mut res = 0u128;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("i128") {
        let mut res = 0i128;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("usize") {
        let mut res = 0usize;
        randomize(&mut res);
        quote! { #res }
    } else if path.is_ident("isize") {
        let mut res = 0isize;
        randomize(&mut res);
        quote! { #res }
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
                    //syn::Type::Array(ref arr) => {
                    //    println!("arr={:?}", arr);
                    //    unreachable!()
                    //},
                    syn::Type::Path(ref pt) => match map_path_to_init_expr(&pt.path) {
                        Some(init) => init,
                        None => return syn::Error::new_spanned(input, "Specified type is not simple integer").to_compile_error().into(),
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
