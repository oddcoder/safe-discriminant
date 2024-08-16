use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Attribute, Error, ItemEnum, Path, Result, Variant};

const PRIM_VALUES: [&str; 10] = [
    "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128",
];

/// checks if `path` is in list `PRIM_VALUES`
fn is_prim(path: &Path) -> bool {
    for pv in PRIM_VALUES {
        if path.is_ident(pv) {
            return true;
        }
    }
    false
}

/// Gets the primitive type x in `#[repr(x)]` from the list of attributes
/// spassed to enums.
/// Returns an error if no valid primitive is found.
fn get_enum_repr_prim(attrs: &[Attribute], error_span: Span) -> Result<Path> {
    let mut prim = None;
    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if is_prim(&meta.path) {
                prim = Some(meta.path);
            }
            Ok(())
        });
    }
    match prim {
        Some(prim) => Ok(prim),
        None => Err(Error::new(
            error_span,
            format!(
                "Discriminant requires a `#[repr(x)] where x is one of {}.",
                PRIM_VALUES.join(", ")
            ),
        )),
    }
}

fn valiate_single_variant(v: Variant) -> Option<Error> {
    if v.discriminant.is_none() {
        Some(Error::new(
            v.span(),
            "Missing explicit discriminant. Note: If you want to reason about \
            discriminants, but do not care about concrete values, consider \
            using `core::mem::Discriminant` instead.",
        ))
    } else {
        None
    }
}

/// Validates that all enum variants have discriminants. If more than one
/// variant is missing a discriminant, errors shall be aggregated.
fn validate_all_variants(variants: impl Iterator<Item = Variant>) -> Result<()> {
    variants
        .filter_map(valiate_single_variant)
        .reduce(|mut acc, e| {
            acc.combine(e);
            acc
        })
        .map(Err)
        .unwrap_or(Ok(()))
}

/// Constructs Discriminant trait implementation for given enum.
/// Returns error in one of two cases:
/// 1- No valid `#[repr(x)]` is found.
/// 2- Any of the enum variants is missing discriminant.
fn derive_discriminant_inner(tagged_enum: ItemEnum) -> Result<TokenStream> {
    let prim = get_enum_repr_prim(&tagged_enum.attrs, tagged_enum.ident.span())?;
    validate_all_variants(tagged_enum.variants.into_iter())?;
    let name = tagged_enum.ident;
    let generics = tagged_enum.generics;
    let derive = quote! {
        unsafe impl #generics safe_discriminant::Discriminant for #name #generics {
            type Repr = #prim;
        }
    };
    Ok(derive.into())
}

/// Top level derive macro for `Discriminant` trait. For more information
/// on how to use refer to `safe-discriminant` crate.
#[proc_macro_derive(Discriminant)]
pub fn derive_discriminant(item: TokenStream) -> TokenStream {
    let tagged_enum = parse_macro_input!(item as ItemEnum);
    match derive_discriminant_inner(tagged_enum) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s,
    }
}
