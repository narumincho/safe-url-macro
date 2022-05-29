#[proc_macro]
pub fn safe_url(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<syn::LitStr>(input) {
        Ok(literal) => {
            let string_literal_value = literal.value();
            match url::Url::parse(&string_literal_value) {
                Ok(_safe_url) => quote::quote! {
                    url::Url::parse(#string_literal_value).unwrap()
                }
                .into(),
                Err(error) => {
                    let message =
                        string_literal_value + " is not valid url. (" + &error.to_string() + ")";
                    quote::quote! {compile_error!(#message)}.into()
                }
            }
        }
        Err(_) => quote::quote! {compile_error!("safe_url macro only support string literal.\n\nexample:\nsafe_url!(\"https://example.com\")\n")}.into(),
    }
}
