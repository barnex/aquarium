use quote::quote;
use std::str::FromStr;
use syn::LitStr;

pub fn str16_impl(arg: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let arg: LitStr = syn::parse2(arg).expect("need single string argument");

    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, va, vb, vc, vd, ve, vf] = fixed_str::Str16::from_str(&arg.value()).expect("valid fixed str16").into_array();

    let mut output = proc_macro2::TokenStream::new();
    // 🪲 should use handle::Handle so we don't depend on import
    output.extend(quote! { fixed_str::Str16::from_bytes([#v0, #v1, #v2, #v3, #v4, #v5, #v6, #v7, #v8, #v9, #va, #vb, #vc, #vd, #ve, #vf])});

    output
}

const N_BYTES: usize = 16;
const MAX_LEN: usize = 15; // just some headroom

// duplicated from fixed_str.rs to avoid cyclic dependency
// (or the need to make a separate crate just for Str::from_str)
fn from_str(s: &str) -> [u8; N_BYTES] {
    let src = s.as_bytes();
    let mut bytes = [0u8; N_BYTES];
    if src.len() > MAX_LEN {
        panic!("handle too long: {s}, must be <= {} characters", MAX_LEN)
    }
    let n = usize::min(src.len(), bytes.len());
    bytes[..n].clone_from_slice(&src[..n]);
    bytes
}
