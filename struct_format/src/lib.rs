extern crate proc_macro;
use lazy_static::lazy_static;
use proc_macro::TokenStream;

use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

lazy_static! {
    static ref RE: Regex = Regex::new("_(.)").unwrap();
}
/**
为CssStyle类生成一个函数`to_css_str()`

CssStyle类具有若干个Option<String>类型的变量。通过to_css_str()函数，输出一个css风格的字符串（-链接的key，;分割的kv）,同时，值为None的成员不输出。

*/
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
                            #struct_str.push_str(format!("{}:{};",stringify!(#f).replace("_","-"),&self.#f.as_ref().unwrap()).as_str());
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

#[proc_macro_derive(parse_node_type)]
pub fn derive_node_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let struct_name_str = struct_name.to_string();
    let sstr = struct_name_str.replace("_","-");
    let expended = quote! {
        // #[wasm_bindgen]
        // impl #struct_name{
        //     #[wasm_bindgen(getter = type)]
        //     pub fn get_type(&self)->String{
        //         return String::from(stringify!(#struct_name));
        //     }
        // }
        // impl ParseNodeToAny for #struct_name{
        //     fn as_any(&self) -> &dyn Any{
        //         &self
        //     }
        // }
        impl ParseNodeToAny for #struct_name{
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_mut_any(&mut self) -> &mut dyn Any {
                self
            }
        }
        impl AnyParseNode for #struct_name{

            fn get_type(&self)->&str{
                return #sstr;
            }
        }
    };
    expended.into()
}

#[proc_macro_derive(html_dom_node)]
pub fn derive_html_dom_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expended = if let Data::Struct(r#struct) = input.data {
        if let Fields::Named(ref fields_name) = r#struct.fields {
            let get_mut_children = if fields_name
                .named
                .iter()
                .any(|field| field.ident.as_ref().unwrap().to_string() == "children")
            {
                quote! {
                         fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>>{
                            return Some(&mut self.children);
                        }
                         fn get_children(&self) -> Option<&Vec<Box<dyn HtmlDomNode>>>{
                            return Some(&self.children);
                        }

                }
            } else {
                quote! {
                         fn get_mut_children(&mut self) -> Option<&mut Vec<Box<dyn HtmlDomNode>>>{
                            return None;
                        }
                         fn get_children(&self) -> Option<&Vec<Box<dyn HtmlDomNode>>>{
                            return None;
                        }

                }
            };
            quote! {

            impl HtmlDomNode for #struct_name {

                #get_mut_children

                fn get_classes(&self) -> &Vec<String> {
                    return &self.classes;
                }
                fn get_mut_classes(&mut self) -> &mut Vec<String> {
                    return &mut self.classes;
                }
                fn set_classes(&mut self, _classes: Vec<String>) {
                    self.classes = _classes;
                }

                fn get_height(&self) -> f64 {
                    return self.height;
                }
                fn set_height(&mut self, _height: f64) {
                    self.height = _height;
                }

                fn get_depth(&self) -> f64 {
                    return self.depth;
                }

                fn set_depth(&mut self, _depth: f64) {
                    self.depth = _depth;
                }

                fn get_max_font_size(&self) -> f64 {
                    return self.max_font_size;
                }
                fn set_max_font_size(&mut self, _max_font_size: f64) {
                    self.max_font_size = _max_font_size;
                }

                fn get_style(&self) -> &CssStyle {
                    return &self.style;
                }
                fn get_mut_style(&mut self) -> &mut CssStyle {
                    return &mut self.style;
                }
                fn set_style(&mut self, _style: CssStyle) {
                    self.style = _style;
                }

                fn has_class(&self, class_name: &String) -> bool {
                    return self.classes.contains(class_name);
                }
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
