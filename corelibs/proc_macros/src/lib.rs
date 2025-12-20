#[proc_macro_derive(Setters, attributes(inspect))]
pub fn derive_setters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macros_impl::derive_setters(input.into()).into()
}

/// Compile-time equivalent of `Str::from_str`
#[proc_macro]
pub fn str16(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macros_impl::str16_impl(input.into()).into()
}
