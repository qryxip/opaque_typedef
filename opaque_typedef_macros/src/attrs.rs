//! Utilities for attributes.

use syn;

/// Checks whether the attribute has the given path.
pub fn is_attr_with_path(attr: &syn::Attribute, path: &[&str]) -> bool {
    attr.path
        .segments
        .iter()
        .map(|seg| &seg.ident)
        .eq(path.iter().cloned())
}

/// Checks whether the word meta item with the given path is specified.
pub fn has_word_meta(meta: &syn::Meta, ident_path: &[&str]) -> bool {
    match *meta {
        syn::Meta::Word(ref ident) => ident_path.len() == 1 && ident == ident_path[0],
        syn::Meta::List(ref metalist) => {
            if ident_path.len() > 1 && metalist.ident == ident_path[0] {
                metalist
                    .nested
                    .iter()
                    .filter_map(|nested_meta| match *nested_meta {
                        syn::NestedMeta::Meta(ref meta) => Some(meta),
                        syn::NestedMeta::Literal(..) => None,
                    })
                    .any(|meta| has_word_meta(meta, &ident_path[1..]))
            } else {
                false
            }
        }
        syn::Meta::NameValue(..) => false,
    }
}

pub fn get_meta_content_by_path(meta: syn::Meta, path: &[&str]) -> Vec<syn::NestedMeta> {
    let mut res = Vec::new();
    append_meta_content_by_path(meta, path, &mut res);
    res
}

fn append_meta_content_by_path(meta: syn::Meta, path: &[&str], vec: &mut Vec<syn::NestedMeta>) {
    if path.is_empty() {
        return;
    }
    if let syn::Meta::List(metalist) = meta {
        if metalist.ident == path[0] {
            append_meta_items_by_path(metalist.nested, &path[1..], vec);
        }
    }
}

fn append_meta_items_by_path<I>(nested_items: I, path: &[&str], vec: &mut Vec<syn::NestedMeta>)
where
    I: IntoIterator<Item = syn::NestedMeta>,
{
    if path.is_empty() {
        vec.extend(nested_items);
        return;
    }
    assert!(!path.is_empty());
    for nested_meta in nested_items {
        if let syn::NestedMeta::Meta(meta) = nested_meta {
            append_meta_content_by_path(meta, path, vec);
        }
    }
}
