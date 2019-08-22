//! Custom derives for easy opaque typedef.
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::type_props::{Sizedness, TypeProps};

mod attrs;
mod derives;
mod type_props;
mod utils;

/// The entrypoint for a `#[derive(OpaqueTypedef)]`-ed type.
#[proc_macro_derive(OpaqueTypedef, attributes(opaque_typedef))]
pub fn opaque_typedef(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = gen_opaque_typedef_impls(&input, Sizedness::Sized);
    gen.into()
}

/// The entrypoint for a `#[derive(OpaqueTypedefUnsized)]`-ed type.
#[proc_macro_derive(OpaqueTypedefUnsized, attributes(opaque_typedef))]
pub fn opaque_typedef_unsized(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = gen_opaque_typedef_impls(&input, Sizedness::Unsized);
    gen.into()
}

/// Generates additional impls for a `#[derive(OpaqueTypedef*)]`-ed type.
fn gen_opaque_typedef_impls(input: &DeriveInput, sizedness: Sizedness) -> TokenStream {
    let props = TypeProps::load(&input, sizedness);
    props.gen_impls()
}
