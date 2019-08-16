extern crate proc_macro;

use proc_macro2::{Span, Ident};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Foo)]
pub fn derive_foo(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let foo = proc_macro_crate::crate_name("foo").unwrap();
    let foo = Ident::new(&foo, Span::call_site());

    let output = quote! {
        const _: () = {
            // 呼び出し側の型名前空間で `#foo` が上書きされている可能性があるので別名でインポートする
            extern crate #foo as _foo;

            impl _foo::Foo for #name {}
        };
    };

    output.into()
}
