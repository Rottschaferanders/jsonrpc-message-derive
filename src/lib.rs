use proc_macro::TokenStream;
use quote::{quote, format_ident};
// use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field};
use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field, DataEnum, Variant};


// #[proc_macro_derive(JsonRpcMessage)]
// pub fn jsonrpc_message_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;

//     let fields = match input.data {
//         Data::Struct(data) => match data.fields {
//             Fields::Named(fields) => fields,
//             _ => panic!("JsonRpcMessage can only be derived for structs with named fields"),
//         },
//         _ => panic!("JsonRpcMessage can only be derived for structs"),
//     };

//     let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

//     if id_field.is_none() {
//         panic!("JsonRpcMessage requires an 'id' field");
//     }

//     let id_field = id_field.unwrap();

//     let id_type = &id_field.ty;

//     let expanded = quote! {
//         impl JsonRpcMessage for #name {
//             fn id(&self) -> Option<#id_type> {
//                 self.id
//             }

//             fn set_id(&mut self, id: #id_type) {
//                 self.id = id;
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }


#[proc_macro_derive(JsonRpcMessage)]
pub fn jsonrpc_message_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Struct(data) => {
            let fields = match data.fields {
                Fields::Named(fields) => fields,
                _ => panic!("JsonRpcMessage can only be derived for structs with named fields"),
            };

            let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

            if id_field.is_none() {
                panic!("JsonRpcMessage requires an 'id' field");
            }

            let id_field = id_field.unwrap();

            let id_type = &id_field.ty;

            quote! {
                impl JsonRpcMessage for #name {
                    fn id(&self) -> Option<#id_type> {
                        Some(self.id)
                    }

                    fn set_id(&mut self, id: #id_type) {
                        self.id = id;
                    }
                }
            }
        }
        Data::Enum(data) => {
            let variants = data.variants;

            let match_arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let fields = match &variant.fields {
                    Fields::Named(fields) => fields,
                    _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
                };

                let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

                if id_field.is_none() {
                    panic!("JsonRpcMessage requires an 'id' field in each enum variant");
                }

                let id_field = id_field.unwrap();

                let id_type = &id_field.ty;

                quote! {
                    #name::#variant_name { id, .. } => Some(*id),
                }
            });

            let set_id_match_arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let fields = match &variant.fields {
                    Fields::Named(fields) => fields,
                    _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
                };

                let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

                if id_field.is_none() {
                    panic!("JsonRpcMessage requires an 'id' field in each enum variant");
                }

                let id_field = id_field.unwrap();

                let id_type = &id_field.ty;

                quote! {
                    #name::#variant_name { id, .. } => *id = new_id,
                }
            });

            quote! {
                impl JsonRpcMessage for #name {
                    fn id(&self) -> Option<u32> {
                        match self {
                            #(#match_arms)*
                            _ => None,
                        }
                    }

                    fn set_id(&mut self, new_id: u32) {
                        match self {
                            #(#set_id_match_arms)*
                            _ => {},
                        }
                    }
                }
            }
        }
        _ => panic!("JsonRpcMessage can only be derived for structs or enums"),
    };

    TokenStream::from(expanded)
}