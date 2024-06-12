use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field};

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

    match input.data {
        Data::Struct(data) => {
            let fields = match data.fields {
                Fields::Named(fields) => fields,
                _ => panic!("JsonRpcMessage can only be derived for structs with named fields"),
            };

            let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

            let id_type = match id_field {
                Some(field) => &field.ty,
                None => panic!("JsonRpcMessage requires an 'id' field"),
            };

            let id_impl = if id_type == &syn::parse_quote!(Option<u32>) {
                quote! {
                    fn id(&self) -> Option<u32> {
                        self.id
                    }

                    fn set_id(&mut self, id: u32) {
                        self.id = Some(id);
                    }
                }
            } else if id_type == &syn::parse_quote!(u32) {
                quote! {
                    fn id(&self) -> Option<u32> {
                        Some(self.id)
                    }

                    fn set_id(&mut self, id: u32) {
                        self.id = id;
                    }
                }
            } else {
                panic!("JsonRpcMessage only supports 'id' fields of type 'u32' or 'Option<u32>'");
            };

            let expanded = quote! {
                impl JsonRpcMessage for #name {
                    #id_impl
                }
            };

            TokenStream::from(expanded)
        }
        Data::Enum(data) => {
            let variants = data.variants;
            let id_impl = quote! {
                fn id(&self) -> Option<u32> {
                    match self {
                        #(Self::#variants { id, .. } => Some(*id),)*
                        _ => None,
                    }
                }
                fn set_id(&mut self, id: u32) {
                    match self {
                        #(Self::#variants { id, .. } => *id = id,)*
                        _ => {}
                    }
                }
            };
            let expanded = quote! {
                impl JsonRpcMessage for #name {
                    #id_impl
                }
            };

            TokenStream::from(expanded)
        },
        _ => panic!("JsonRpcMessage can only be derived for structs"),
    }
}