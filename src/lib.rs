use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field};

#[proc_macro_derive(JsonRpcMessage)]
pub fn jsonrpc_message_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("JsonRpcMessage can only be derived for structs with named fields"),
        },
        _ => panic!("JsonRpcMessage can only be derived for structs"),
    };

    let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

    if id_field.is_none() {
        panic!("JsonRpcMessage requires an 'id' field");
    }

    let id_field = id_field.unwrap();

    let id_type = &id_field.ty;

    let expanded = quote! {
        impl JsonRpcMessage for #name {
            fn id(&self) -> Option<#id_type> {
                self.id
            }

            fn set_id(&mut self, id: #id_type) {
                self.id = id;
            }
        }
    };

    TokenStream::from(expanded)
}