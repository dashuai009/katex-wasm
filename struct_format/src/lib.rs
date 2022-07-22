extern crate proc_macro;
use lazy_static::lazy_static;
use proc_macro::TokenStream;

use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

lazy_static! {
    static ref RE: Regex = Regex::new("_(.)").unwrap();
}

#[proc_macro_derive(format)]
pub fn derive_format(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let struct_str = Ident::new("struct_str", struct_name.span());
    let expended = if let Data::Struct(r#struct) = input.data {
        if let Fields::Named(ref fields_name) = r#struct.fields {
            // let get_selfs: Vec<_> = fields_name
            //     .named
            //     .iter()
            //     .map(|field| {
            //         let f = field.ident.as_ref().unwrap();
            //
            //         quote! {
            //             stringify!(#f),&self.#f
            //         }
            //     })
            //     .collect();
            // let format_string = "{}:{};".repeat(get_selfs.len());
            // let format_literal = proc_macro2::Literal::string(format_string.as_str());
            // let struct_fields = quote! {
            //     #(#get_selfs),*
            // };
            let get_some: Vec<_> = fields_name
                .named
                .iter()
                .map(|field| {
                    let f = field.ident.as_ref().unwrap();

                    quote! {
                        if self.#f.is_some(){
                            #struct_str.push_str(format!("{}:{};",stringify!(#f),&self.#f.as_ref().unwrap()).as_str());
                        }
                    }
                })
                .collect();

            let get_underline_fields: Vec<_> = fields_name
                .named
                .iter()
                .filter(|field| {
                    let f = field.ident.as_ref().unwrap();
                    f.to_string().contains("_")
                })
                .map(|field| {
                    // let t = f
                    let f = field.ident.as_ref().unwrap();
                    let f_str = f.to_string();
                    let g_name = RE.replace_all(f_str.as_str(), |captures: &regex::Captures| {
                        captures[1].to_uppercase()
                    });
                    let g = Ident::new(&*g_name, struct_name.span());

                    let s = Ident::new(format!("set_{}", f_str).as_str(), struct_name.span());
                    quote! {
                        #[wasm_bindgen(getter = #g)]
                        pub fn #f(&self)->Option<String>{
                            self.#f.clone()
                        }
                         #[wasm_bindgen(setter = #g)]
                        pub fn #s(&mut self,x:Option<String>){
                            self.#f = x;
                        }
                    }
                })
                .collect();

            quote! {
                // impl std::fmt::Display for #struct_name{
                //     fn fmt(&self,f:&mut std::fmt::Formatter)->std::fmt::Result{
                //         write!(f , #format_literal , #struct_fields)
                //     }
                // }
                impl #struct_name{
                    pub fn to_css_str(&self)->String{
                        let mut #struct_str = String::new();
                        #(#get_some)*
                        #struct_str
                    }
                }
                #[wasm_bindgen]
                impl #struct_name{
                    #(#get_underline_fields)*
                }
            }
        } else {
            panic!("sorry, may it's a complicated struct.")
        }
    } else {
        panic!("sorry, Show is not implemented for union or enum type.")
    };
    expended.into()
}
