//! Impl generators for `std::cmp::Partial{Eq,Ord}` traits.

use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::type_props::{CmpSpec, Sizedness, TypeProps};
use crate::utils::extend_generics;

use super::Derive;

pub fn gen_impl_ord(props: &TypeProps) -> TokenStream {
    let ty_outer = &props.ty_outer;
    let type_generics = &props.type_generics;
    let ty_outer_generic = quote!(#ty_outer #type_generics);
    let ty_inner = props.field_inner.ty();
    let self_as_inner = props.tokens_outer_expr_as_inner(quote!(self));
    let other_as_inner = props.tokens_outer_expr_as_inner(quote!(other));

    CmpImplSpec {
        type_props: &props,
        generics: props.generics,
        trait_spec: CmpTraitSpec::Ord,
        cmp_spec: &props.cmp_spec,
        ty_inner,
        ty_lhs: &ty_outer_generic,
        lhs_self_as_inner: &self_as_inner,
        ty_rhs: &ty_outer_generic,
        rhs_other_as_inner: &other_as_inner,
    }
    .gen_impl()
}

/// Generates an impl for the target.
pub fn gen_impl_partial_cmp(target: Derive, props: &TypeProps) -> TokenStream {
    let trait_spec = match target {
        Derive::PartialEqInner
        | Derive::PartialEqInnerRev
        | Derive::PartialEqInnerCow
        | Derive::PartialEqInnerCowRev
        | Derive::PartialEqSelf
        | Derive::PartialEqSelfCow
        | Derive::PartialEqSelfCowRev
        | Derive::PartialEqSelfCowAndInner
        | Derive::PartialEqSelfCowAndInnerRev => CmpTraitSpec::PartialEq,
        Derive::PartialOrdInner
        | Derive::PartialOrdInnerRev
        | Derive::PartialOrdInnerCow
        | Derive::PartialOrdInnerCowRev
        | Derive::PartialOrdSelf
        | Derive::PartialOrdSelfCow
        | Derive::PartialOrdSelfCowRev
        | Derive::PartialOrdSelfCowAndInner
        | Derive::PartialOrdSelfCowAndInnerRev => CmpTraitSpec::PartialOrd,
        _ => unreachable!("Should never happen"),
    };
    let ty_outer = &props.ty_outer;
    let type_generics = &props.type_generics;
    let ty_inner = props.field_inner.ty();
    let self_as_inner = props.tokens_outer_expr_as_inner(quote!(self));
    let other_as_inner = props.tokens_outer_expr_as_inner(quote!(other));
    match target {
        Derive::PartialEqInner | Derive::PartialOrdInner => {
            let inner_and_outer = CmpImplSpec {
                type_props: &props,
                generics: props.generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(#ty_outer #type_generics),
                lhs_self_as_inner: &self_as_inner,
                ty_rhs: ty_inner,
                rhs_other_as_inner: &quote!(other),
            }
            .gen_impl();
            let extra = if props.inner_sizedness == Sizedness::Sized && props.has_type_params() {
                quote!()
            } else {
                let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
                let new_lt = &new_lts[0];
                let inner_and_outer_ref = CmpImplSpec {
                    type_props: &props,
                    generics: &generics,
                    trait_spec,
                    cmp_spec: &props.cmp_spec,
                    ty_inner,
                    ty_lhs: quote!(&#new_lt #ty_outer #type_generics),
                    lhs_self_as_inner: &props.tokens_outer_expr_as_inner(quote!(*self)),
                    ty_rhs: ty_inner,
                    rhs_other_as_inner: &quote!(other),
                }
                .gen_impl();
                let inner_ref_and_outer = CmpImplSpec {
                    type_props: &props,
                    generics: &generics,
                    trait_spec,
                    cmp_spec: &props.cmp_spec,
                    ty_inner,
                    ty_lhs: quote!(#ty_outer #type_generics),
                    lhs_self_as_inner: &self_as_inner,
                    ty_rhs: quote!(&#new_lt #ty_inner),
                    rhs_other_as_inner: &quote!(*other),
                }
                .gen_impl();
                quote! {
                    #inner_and_outer_ref
                    #inner_ref_and_outer
                }
            };
            quote! {
                #inner_and_outer
                #extra
            }
        }
        Derive::PartialEqInnerRev | Derive::PartialOrdInnerRev => {
            let inner_and_outer_rev = CmpImplSpec {
                type_props: &props,
                generics: props.generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: ty_inner,
                lhs_self_as_inner: &quote!(self),
                ty_rhs: quote!(#ty_outer #type_generics),
                rhs_other_as_inner: &other_as_inner,
            }
            .gen_impl();
            let extra = if props.inner_sizedness == Sizedness::Sized && props.has_type_params() {
                quote!()
            } else {
                let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
                let new_lt = &new_lts[0];
                let inner_and_outer_ref_rev = CmpImplSpec {
                    type_props: &props,
                    generics: &generics,
                    trait_spec,
                    cmp_spec: &props.cmp_spec,
                    ty_inner,
                    ty_lhs: ty_inner,
                    lhs_self_as_inner: &quote!(self),
                    ty_rhs: quote!(&#new_lt #ty_outer #type_generics),
                    rhs_other_as_inner: &props.tokens_outer_expr_as_inner(quote!(*other)),
                }
                .gen_impl();
                let inner_ref_and_outer_rev = CmpImplSpec {
                    type_props: &props,
                    generics: &generics,
                    trait_spec,
                    cmp_spec: &props.cmp_spec,
                    ty_inner,
                    ty_lhs: quote!(&#new_lt #ty_inner),
                    lhs_self_as_inner: &quote!(*self),
                    ty_rhs: quote!(#ty_outer #type_generics),
                    rhs_other_as_inner: &other_as_inner,
                }
                .gen_impl();
                quote! {
                    #inner_and_outer_ref_rev
                    #inner_ref_and_outer_rev
                }
            };
            quote! {
                #inner_and_outer_rev
                #extra
            }
        }
        Derive::PartialEqInnerCow | Derive::PartialOrdInnerCow => {
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
            let new_lt = &new_lts[0];
            let inner_cow_and_outer = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(::std::borrow::Cow<#new_lt, #ty_inner>),
                lhs_self_as_inner: &quote!(&*self),
                ty_rhs: quote!(#ty_outer #type_generics),
                rhs_other_as_inner: &other_as_inner,
            }
            .gen_impl();
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 2, &[]);
            let new_lt0 = &new_lts[0];
            let new_lt1 = &new_lts[1];
            let inner_cow_and_outer_ref = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(::std::borrow::Cow<#new_lt0, #ty_inner>),
                lhs_self_as_inner: &quote!(&*self),
                ty_rhs: quote!(&#new_lt1 #ty_outer #type_generics),
                rhs_other_as_inner: &props.tokens_outer_expr_as_inner(quote!(*other)),
            }
            .gen_impl();
            quote! {
                #inner_cow_and_outer
                #inner_cow_and_outer_ref
            }
        }
        Derive::PartialEqInnerCowRev | Derive::PartialOrdInnerCowRev => {
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
            let new_lt = &new_lts[0];
            let inner_cow_and_outer_rev = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(#ty_outer #type_generics),
                lhs_self_as_inner: &self_as_inner,
                ty_rhs: quote!(::std::borrow::Cow<#new_lt, #ty_inner>),
                rhs_other_as_inner: &quote!(&*other),
            }
            .gen_impl();
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 2, &[]);
            let new_lt0 = &new_lts[0];
            let new_lt1 = &new_lts[1];
            let inner_cow_and_outer_ref_rev = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(&#new_lt0 #ty_outer #type_generics),
                lhs_self_as_inner: &props.tokens_outer_expr_as_inner(quote!(*self)),
                ty_rhs: quote!(::std::borrow::Cow<#new_lt1, #ty_inner>),
                rhs_other_as_inner: &quote!(&*other),
            }
            .gen_impl();
            quote! {
                #inner_cow_and_outer_rev
                #inner_cow_and_outer_ref_rev
            }
        }
        Derive::PartialEqSelf | Derive::PartialOrdSelf => {
            let outer_and_outer = CmpImplSpec {
                type_props: &props,
                generics: props.generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(#ty_outer #type_generics),
                lhs_self_as_inner: &self_as_inner,
                ty_rhs: quote!(#ty_outer #type_generics),
                rhs_other_as_inner: &other_as_inner,
            }
            .gen_impl();
            quote! {
                #outer_and_outer
            }
        }
        Derive::PartialEqSelfCow | Derive::PartialOrdSelfCow => {
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
            let new_lt = &new_lts[0];
            let outer_cow_and_outer = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(::std::borrow::Cow<#new_lt, #ty_outer #type_generics>),
                lhs_self_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*self)),
                ty_rhs: quote!(#ty_outer #type_generics),
                rhs_other_as_inner: &other_as_inner,
            }
            .gen_impl();
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 2, &[]);
            let new_lt0 = &new_lts[0];
            let new_lt1 = &new_lts[1];
            let outer_cow_and_outer_ref = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(::std::borrow::Cow<#new_lt0, #ty_outer #type_generics>),
                lhs_self_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*self)),
                ty_rhs: quote!(&#new_lt1 #ty_outer #type_generics),
                rhs_other_as_inner: &props.tokens_outer_expr_as_inner(quote!(*other)),
            }
            .gen_impl();
            quote! {
                #outer_cow_and_outer
                #outer_cow_and_outer_ref
            }
        }
        Derive::PartialEqSelfCowRev | Derive::PartialOrdSelfCowRev => {
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
            let new_lt = &new_lts[0];
            let outer_cow_and_outer_rev = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(#ty_outer #type_generics),
                lhs_self_as_inner: &self_as_inner,
                ty_rhs: quote!(::std::borrow::Cow<#new_lt, #ty_outer #type_generics>),
                rhs_other_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*other)),
            }
            .gen_impl();
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 2, &[]);
            let new_lt0 = &new_lts[0];
            let new_lt1 = &new_lts[1];
            let outer_cow_and_outer_ref_rev = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(&#new_lt0 #ty_outer #type_generics),
                lhs_self_as_inner: &props.tokens_outer_expr_as_inner(quote!(*self)),
                ty_rhs: quote!(::std::borrow::Cow<#new_lt1, #ty_outer #type_generics>),
                rhs_other_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*other)),
            }
            .gen_impl();
            quote! {
                #outer_cow_and_outer_rev
                #outer_cow_and_outer_ref_rev
            }
        }
        Derive::PartialEqSelfCowAndInner | Derive::PartialOrdSelfCowAndInner => {
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
            let new_lt = &new_lts[0];
            let outer_cow_and_inner = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(::std::borrow::Cow<#new_lt, #ty_outer #type_generics>),
                lhs_self_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*self)),
                ty_rhs: ty_inner,
                rhs_other_as_inner: &quote!(other),
            }
            .gen_impl();
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 2, &[]);
            let new_lt0 = &new_lts[0];
            let new_lt1 = &new_lts[1];
            let outer_cow_and_inner_ref = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(::std::borrow::Cow<#new_lt0, #ty_outer #type_generics>),
                lhs_self_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*self)),
                ty_rhs: quote!(&#new_lt1 #ty_inner),
                rhs_other_as_inner: &quote!(*other),
            }
            .gen_impl();
            quote! {
                #outer_cow_and_inner
                #outer_cow_and_inner_ref
            }
        }
        Derive::PartialEqSelfCowAndInnerRev | Derive::PartialOrdSelfCowAndInnerRev => {
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 1, &[]);
            let new_lt = &new_lts[0];
            let outer_cow_and_inner_rev = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: ty_inner,
                lhs_self_as_inner: &quote!(self),
                ty_rhs: quote!(::std::borrow::Cow<#new_lt, #ty_outer #type_generics>),
                rhs_other_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*other)),
            }
            .gen_impl();
            let (generics, new_lts) = extend_generics(Cow::Borrowed(props.generics), 2, &[]);
            let new_lt0 = &new_lts[0];
            let new_lt1 = &new_lts[1];
            let outer_cow_and_inner_ref_rev = CmpImplSpec {
                type_props: &props,
                generics: &generics,
                trait_spec,
                cmp_spec: &props.cmp_spec,
                ty_inner,
                ty_lhs: quote!(&#new_lt0 #ty_inner),
                lhs_self_as_inner: &quote!(*self),
                ty_rhs: quote!(::std::borrow::Cow<#new_lt1, #ty_outer #type_generics>),
                rhs_other_as_inner: &props.tokens_outer_expr_as_inner(quote!(&*other)),
            }
            .gen_impl();
            quote! {
                #outer_cow_and_inner_rev
                #outer_cow_and_inner_ref_rev
            }
        }
        _ => unreachable!("Should never happen"),
    }
}

#[derive(Debug, Clone, Copy)]
enum CmpTraitSpec {
    PartialEq,
    PartialOrd,
    Ord,
}

impl CmpTraitSpec {
    pub fn target_trait(self) -> TokenStream {
        match self {
            CmpTraitSpec::PartialEq => quote!(::std::cmp::PartialEq),
            CmpTraitSpec::PartialOrd => quote!(::std::cmp::PartialOrd),
            CmpTraitSpec::Ord => quote!(::std::cmp::Ord),
        }
    }

    pub fn method_name(self) -> TokenStream {
        match self {
            CmpTraitSpec::PartialEq => quote!(eq),
            CmpTraitSpec::PartialOrd => quote!(partial_cmp),
            CmpTraitSpec::Ord => quote!(cmp),
        }
    }

    pub fn comparator(self, cmp_spec: &CmpSpec) -> TokenStream {
        match self {
            CmpTraitSpec::PartialEq => cmp_spec.partial_eq(),
            CmpTraitSpec::PartialOrd => cmp_spec.partial_ord(),
            CmpTraitSpec::Ord => cmp_spec.ord(),
        }
    }

    pub fn ty_ret(self) -> TokenStream {
        match self {
            CmpTraitSpec::PartialEq => quote!(bool),
            CmpTraitSpec::PartialOrd => quote!(Option<::std::cmp::Ordering>),
            CmpTraitSpec::Ord => quote!(::std::cmp::Ordering),
        }
    }
}

#[derive(Clone)]
struct CmpImplSpec<'a, TyI, TyL, TyR> {
    type_props: &'a TypeProps<'a>,
    generics: &'a syn::Generics,
    trait_spec: CmpTraitSpec,
    cmp_spec: &'a CmpSpec,
    ty_inner: TyI,
    ty_lhs: TyL,
    lhs_self_as_inner: &'a TokenStream,
    ty_rhs: TyR,
    rhs_other_as_inner: &'a TokenStream,
}

impl<'a, TyI, TyL, TyR> CmpImplSpec<'a, TyI, TyL, TyR>
where
    TyI: ToTokens,
    TyL: ToTokens,
    TyR: ToTokens,
{
    fn gen_impl(&self) -> TokenStream {
        let CmpImplSpec {
            type_props,
            generics,
            ref trait_spec,
            ref cmp_spec,
            ref ty_inner,
            ref ty_lhs,
            lhs_self_as_inner,
            ref ty_rhs,
            rhs_other_as_inner,
        } = *self;
        let target_trait = trait_spec.target_trait();
        let extra_preds = if type_props.has_type_params() {
            let ty_inner = ty_inner.into_token_stream();
            let pred = match *trait_spec {
                CmpTraitSpec::PartialEq | CmpTraitSpec::PartialOrd => {
                    syn::parse_str::<syn::WherePredicate>(&format!(
                        "{}: {}<{}>",
                        ty_inner, target_trait, ty_inner
                    ))
                    .expect("Failed to generate `WherePredicate`")
                }
                CmpTraitSpec::Ord => syn::parse_str::<syn::WherePredicate>(&format!(
                    "{}: {}",
                    ty_inner, target_trait
                ))
                .expect("Failed to generate `WherePredicate`"),
            };
            vec![pred]
        } else {
            Vec::new()
        };
        let method_name = trait_spec.method_name();
        let fn_cmp = trait_spec.comparator(cmp_spec);
        let ty_ret = trait_spec.ty_ret();
        let (generics, _) = extend_generics(Cow::Borrowed(generics), 0, &extra_preds);
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let target = match *trait_spec {
            CmpTraitSpec::PartialEq | CmpTraitSpec::PartialOrd => quote!(#target_trait<#ty_rhs>),
            CmpTraitSpec::Ord => quote!(#target_trait),
        };
        quote! {
            impl #impl_generics #target for #ty_lhs #where_clause {
                fn #method_name(&self, other: &#ty_rhs) -> #ty_ret {
                    #fn_cmp(#lhs_self_as_inner as &#ty_inner, #rhs_other_as_inner as &#ty_inner)
                }
            }
        }
    }
}
