use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn do_n_times(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = syn::parse_macro_input!(item as syn::ItemFn);

    let n: syn::Expr = syn::parse_str(&attr.to_string()).unwrap();

    let n = n.to_token_stream().to_string().parse::<usize>().unwrap();

    let function_body = &func.block;
    let mut new_function_body = quote::quote! {};

    for i in 1..n + 1 {
        new_function_body = quote::quote! {
            #new_function_body

            {
                const N: usize = #i;

                #function_body
            }
        };
    }

    let new_function_body = quote::quote! {
        { #new_function_body }
    };

    func.block = Box::new(syn::parse2(new_function_body).unwrap());

    TokenStream::from(quote::quote! {
        #func
    })
}
