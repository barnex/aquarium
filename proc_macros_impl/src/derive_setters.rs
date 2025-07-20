use quote::{format_ident, quote};
use syn::{ItemStruct, Type, TypePath};

pub fn derive_setters(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	let mut output = proc_macro2::TokenStream::new();

	let strct: ItemStruct = syn::parse2(input).expect("parse");
	let strct_type = strct.ident;
	for field in strct.fields.iter() {
		let field_type = &field.ty;
		let field_name = field.ident.as_ref().expect("named field");

		let getter_name = format_ident!("{field_name}");
		let setter_name = format_ident!("set_{field_name}");

		// Setter/Getters for Mut<T>
		if let Some(inner_type) = inner_type(field_type, "Mut") {
			output.extend(quote! {
				impl #strct_type{
					pub fn #getter_name(&self) -> #inner_type{
						self.#field_name.get()
					}

					pub fn #setter_name(&self, v: #inner_type){
						self.#field_name.set(v)
					}
				}
			});
		// Setters/Getters for <primitive>.
		} else if is_simple_type(field_type) {
			output.extend(quote! {
				impl #strct_type{
					pub fn #getter_name(&self) -> #field_type{
						self.#field_name
					}

					pub fn #setter_name(&mut self, v: #field_type){
						self.#field_name = v;
					}
				}
			});
		// Setters/Getters for <NonCopy>
		} else {
			output.extend(quote! {
				impl #strct_type{
					pub fn #getter_name(&self) -> &#field_type{
						&self.#field_name
					}

					pub fn #setter_name(&mut self, v: #field_type){
						self.#field_name = v;
					}
				}
			});
		}
	}
	output
}

/// If `ty` is of the form `Outer<Inner>` ("Outer" passed as string), return `Inner`.
/// E.g.:
///        inner_type(Vec<u32>, "Vec") => Some(u32)
///        inner_type(Vec<u32>, "Box") => None
fn inner_type<'t>(ty: &'t Type, outer: &str) -> Option<&'t Type> {
	if let Type::Path(TypePath { qself: None, path }) = ty {
		if path.segments.len() == 1 {
			let segment = &path.segments[0];
			if segment.ident == outer {
				if let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments {
					if arguments.args.len() == 1 {
						let argument = &arguments.args[0];
						if let syn::GenericArgument::Type(t) = argument {
							return Some(t);
						}
					}
				}
			}
		}
	}
	None
}

fn is_simple_type(ty: &Type) -> bool {
	if let Type::Path(TypePath { qself: None, path }) = ty {
		if path.segments.len() == 1 {
			let segment = &path.segments[0];
			if segment.ident.to_string().chars().all(|c| c == c.to_ascii_lowercase()) {
				return true;
			}
		}
	}
	false
}
