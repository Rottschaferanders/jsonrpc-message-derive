// #![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::{quote, format_ident};
// use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field};
// use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field, DataEnum, Variant};
// use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field, DataEnum, Variant, Type};
// use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Fields, Data, FieldsNamed, Field, DataEnum, Variant, Type, TypePath, PathArguments, PathSegment, GenericArgument};
use std::collections::HashSet;


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


// #[proc_macro_derive(JsonRpcMessage)]
// pub fn jsonrpc_message_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;

//     let expanded = match input.data {
//         Data::Struct(data) => {
//             let fields = match data.fields {
//                 Fields::Named(fields) => fields,
//                 _ => panic!("JsonRpcMessage can only be derived for structs with named fields"),
//             };

//             let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

//             if id_field.is_none() {
//                 panic!("JsonRpcMessage requires an 'id' field");
//             }

//             let id_field = id_field.unwrap();

//             let id_type = &id_field.ty;

//             quote! {
//                 impl JsonRpcMessage for #name {
//                     fn id(&self) -> Option<#id_type> {
//                         Some(self.id)
//                     }

//                     fn set_id(&mut self, id: #id_type) {
//                         self.id = id;
//                     }
//                 }
//             }
//         }
//         Data::Enum(data) => {
//             let variants = data.variants;

//             let match_arms = variants.iter().map(|variant| {
//                 let variant_name = &variant.ident;
//                 let fields = match &variant.fields {
//                     Fields::Named(fields) => fields,
//                     _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
//                 };

//                 let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

//                 if id_field.is_none() {
//                     panic!("JsonRpcMessage requires an 'id' field in each enum variant");
//                 }

//                 let id_field = id_field.unwrap();

//                 let id_type = &id_field.ty;

//                 quote! {
//                     #name::#variant_name { id, .. } => Some(*id),
//                 }
//             });

//             let set_id_match_arms = variants.iter().map(|variant| {
//                 let variant_name = &variant.ident;
//                 let fields = match &variant.fields {
//                     Fields::Named(fields) => fields,
//                     _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
//                 };

//                 let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

//                 if id_field.is_none() {
//                     panic!("JsonRpcMessage requires an 'id' field in each enum variant");
//                 }

//                 let id_field = id_field.unwrap();

//                 let id_type = &id_field.ty;

//                 quote! {
//                     #name::#variant_name { id, .. } => *id = new_id,
//                 }
//             });

//             quote! {
//                 impl JsonRpcMessage for #name {
//                     fn id(&self) -> Option<u32> {
//                         match self {
//                             #(#match_arms)*
//                             _ => None,
//                         }
//                     }

//                     fn set_id(&mut self, new_id: u32) {
//                         match self {
//                             #(#set_id_match_arms)*
//                             _ => {},
//                         }
//                     }
//                 }
//             }
//         }
//         _ => panic!("JsonRpcMessage can only be derived for structs or enums"),
//     };

//     TokenStream::from(expanded)
// }

// #[proc_macro_derive(JsonRpcMessage)]
// pub fn jsonrpc_message_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;

//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

//     let expanded = match input.data {
//         Data::Struct(data) => {
//             let fields = match data.fields {
//                 Fields::Named(fields) => fields,
//                 _ => panic!("JsonRpcMessage can only be derived for structs with named fields"),
//             };

//             let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

//             if id_field.is_none() {
//                 panic!("JsonRpcMessage requires an 'id' field");
//             }

//             let id_field = id_field.unwrap();

//             let id_type = &id_field.ty;

//             quote! {
//                 impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
//                     fn id(&self) -> Option<#id_type> {
//                         self.id
//                     }

//                     fn set_id(&mut self, id: #id_type) {
//                         self.id = id;
//                     }
//                 }
//             }
//         }
//         Data::Enum(data) => {
//             let variants = data.variants.iter().map(|variant| {
//                 let variant_name = &variant.ident;
//                 match &variant.fields {
//                     Fields::Named(fields) => {
//                         let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");
//                         if let Some(id_field) = id_field {
//                             let id_type = &id_field.ty;
//                             quote! {
//                                 #name::#variant_name { id, .. } => Some(*id),
//                             }
//                         } else {
//                             quote! {
//                                 #name::#variant_name { .. } => None,
//                             }
//                         }
//                     }
//                     _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
//                 }
//             });
//             let variants_set_id = data.variants.iter().map(|variant| {
//                 let variant_name = &variant.ident;
//                 match &variant.fields {
//                     Fields::Named(fields) => {
//                         let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");
//                         if let Some(id_field) = id_field {
//                             let id_type = &id_field.ty;
//                             quote! {
//                                 #name::#variant_name { id, .. } => *id = new_id,
//                             }
//                         } else {
//                             quote! {
//                                 #name::#variant_name { .. } => {},
//                             }
//                         }
//                     }
//                     _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
//                 }
//             });

//             quote! {
//                 impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
//                     fn id(&self) -> Option<u32> {
//                         match self {
//                             #(#variants)*
//                         }
//                     }

//                     fn set_id(&mut self, new_id: u32) {
//                         match self {
//                             #(#variants_set_id)*
//                         }
//                     }
//                 }
//             }
//         }
//         _ => panic!("JsonRpcMessage can only be derived for structs and enums"),
//     };

//     TokenStream::from(expanded)
// }

// #[proc_macro_derive(JsonRpcMessage)]
// pub fn jsonrpc_message_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;

//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

//     let expanded = match input.data {
//         Data::Struct(data) => {
//             let fields = match data.fields {
//                 Fields::Named(fields) => fields,
//                 _ => panic!("JsonRpcMessage can only be derived for structs with named fields"),
//             };

//             let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");

//             if id_field.is_none() {
//                 proc_macro::Diagnostic::spanned(
//                     input.span(),
//                     proc_macro::Level::Warning,
//                     "JsonRpcMessage derived for a struct without an 'id' field. This is only valid for JSON-RPC notification messages.",
//                 ).emit();
//             }

//             let id_field = id_field.unwrap_or_else(|| panic!("JsonRpcMessage requires an 'id' field"));

//             let id_type = &id_field.ty;

//             quote! {
//                 impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
//                     fn id(&self) -> Option<u32> {
//                         match self.id {
//                             Some(id) => Some(id as u32),
//                             None => None,
//                         }
//                     }

//                     fn set_id(&mut self, id: u32) {
//                         self.id = Some(id as #id_type);
//                     }
//                 }
//             }
//         }
//         Data::Enum(data) => {
//             let variants = data.variants.iter().map(|variant| {
//                 let variant_name = &variant.ident;
//                 match &variant.fields {
//                     Fields::Named(fields) => {
//                         let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");
//                         if let Some(id_field) = id_field {
//                             let id_type = &id_field.ty;
//                             quote! {
//                                 #name::#variant_name { id, .. } => match id {
//                                     Some(id) => Some(id as u32),
//                                     None => None,
//                                 },
//                             }
//                         } else {
//                             quote! {
//                                 #name::#variant_name { .. } => None,
//                             }
//                         }
//                     }
//                     _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
//                 }
//             });
//             let variants_set_id = data.variants.iter().map(|variant| {
//                 let variant_name = &variant.ident;
//                 match &variant.fields {
//                     Fields::Named(fields) => {
//                         let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");
//                         if let Some(id_field) = id_field {
//                             let id_type = &id_field.ty;
//                             quote! {
//                                 #name::#variant_name { id, .. } => *id = Some(new_id as #id_type),
//                             }
//                         } else {
//                             quote! {
//                                 #name::#variant_name { .. } => {},
//                             }
//                         }
//                     }
//                     _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
//                 }
//             });

//             quote! {
//                 impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
//                     fn id(&self) -> Option<u32> {
//                         match self {
//                             #(#variants)*
//                         }
//                     }

//                     fn set_id(&mut self, new_id: u32) {
//                         match self {
//                             #(#variants_set_id)*
//                         }
//                     }
//                 }
//             }
//         }
//         _ => panic!("JsonRpcMessage can only be derived for structs and enums"),
//     };

//     TokenStream::from(expanded)
// }

#[proc_macro_derive(JsonRpcMessage)]
pub fn jsonrpc_message_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut id_field = None;
    let mut id_type = None;
    let mut has_id = false;

    // match input.data {
    //     Data::Struct(ref data) => {
    //         if let Fields::Named(fields) = data.fields {
    //             for field in fields.named {
    //                 if let Some(ident) = field.ident {
    //                     if ident == "id" {
    //                         id_field = Some(ident);
    //                         id_type = Some(field.ty);
    //                         has_id = true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Data::Enum(ref data) => {
    //         for variant in data.variants {
    //             if let Fields::Named(fields) = variant.fields {
    //                 for field in fields.named {
    //                     if let Some(ident) = field.ident {
    //                         if ident == "id" {
    //                             id_field = Some(ident);
    //                             id_type = Some(field.ty);
    //                             has_id = true;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     _ => panic!("JsonRpcMessage can only be derived for structs and enums"),
    // }

    match input.data {
        Data::Struct(ref data) => {
            if let Fields::Named(ref fields) = data.fields {
                for field in fields.named.iter() {
                    if let Some(ident) = field.ident {
                        if ident == "id" {
                            id_field = Some(ident);
                            id_type = Some(field.ty.clone());
                            has_id = true;
                        }
                    }
                }
            }
        }
        Data::Enum(ref data) => {
            for variant in data.variants.iter() {
                if let Fields::Named(ref fields) = variant.fields {
                    for field in fields.named.iter() {
                        if let Some(ident) = field.ident {
                            if ident == "id" {
                                id_field = Some(ident);
                                id_type = Some(field.ty.clone());
                                has_id = true;
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("JsonRpcMessage can only be derived for structs and enums"),
    }

    if !has_id {
        println!("cargo:warning=JsonRpcMessage derived for {} without an id field. Unless this is intended for parsing JSON-RPC notification messages, consider adding an id field.", name);
    }


    let id_type = if let Some(id_type) = id_type {
        match id_type {
            // Type::Path(TypePath { path, .. }) => {
            Type::Path(TypePath { ref path, .. }) => {
                let segments = path.segments.iter().collect::<Vec<_>>();
                if let Some(PathSegment { ident, arguments }) = segments.last() {
                    match (ident.to_string().as_str(), arguments) {
                        ("Option", PathArguments::AngleBracketed(args)) => {
                            // if let Some(Type::Path(TypePath { path, .. })) = args.args.first() {
                            if let Some(syn::GenericArgument::Type(Type::Path(TypePath { path, .. }))) = args.args.first() {
                                let segments = path.segments.iter().collect::<Vec<_>>();
                                if let Some(PathSegment { ident, .. }) = segments.last() {
                                    match ident.to_string().as_str() {
                                        "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "Uuid" => {
                                            quote! { #id_type }
                                        }
                                        _ => panic!("JsonRpcMessage id field must be an integer type or UUID"),
                                    }
                                } else {
                                    panic!("JsonRpcMessage id field must be an integer type or UUID");
                                }
                            } else {
                                panic!("JsonRpcMessage id field must be an integer type or UUID");
                            }
                        }
                        ("u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "Uuid", _) => {
                            quote! { std::option::Option<#id_type> }
                        }
                        _ => panic!("JsonRpcMessage id field must be an integer type or UUID"),
                    }
                } else {
                    panic!("JsonRpcMessage id field must be an integer type or UUID");
                }
            }
            _ => panic!("JsonRpcMessage id field must be an integer type or UUID"),
        }
    } else {
        quote! { std::option::Option<u32> }
    };

    // let id_type = if let Some(id_type) = id_type {
    //     match id_type {
    //         Type::Path(TypePath { path, .. }) => {
    //             let segments = path.segments.iter().collect::<Vec<_>>();
    //             if let Some(PathSegment { ident, arguments }) = segments.last() {
    //                 match (ident.to_string().as_str(), arguments) {
    //                     ("Option", PathArguments::AngleBracketed(args)) => {
    //                         if let Some(arg) = args.args.first() {
    //                             match arg {
    //                                 GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
    //                                     let segments = path.segments.iter().collect::<Vec<_>>();
    //                                     if let Some(PathSegment { ident, .. }) = segments.last() {
    //                                         match ident.to_string().as_str() {
    //                                             "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "Uuid" => {
    //                                                 quote! { #id_type }
    //                                             }
    //                                             _ => panic!("JsonRpcMessage id field must be an integer type or UUID"),
    //                                         }
    //                                     } else {
    //                                         panic!("JsonRpcMessage id field must be an integer type or UUID");
    //                                     }
    //                                 }
    //                                 _ => panic!("JsonRpcMessage id field must be an integer type or UUID"),
    //                             }
    //                         } else {
    //                             panic!("JsonRpcMessage id field must be an integer type or UUID");
    //                         }
    //                     }
    //                     ("u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "Uuid", _) => {
    //                         quote! { std::option::Option<#id_type> }
    //                     }
    //                     _ => panic!("JsonRpcMessage id field must be an integer type or UUID"),
    //                 }
    //             } else {
    //                 panic!("JsonRpcMessage id field must be an integer type or UUID");
    //             }
    //         }
    //         _ => panic!("JsonRpcMessage id field must be an integer type or UUID"),
    //     }
    // } else {
    //     quote! { std::option::Option<u32> }
    // };

    // let expanded = quote! {
    //     impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
    //         fn id(&self) -> #id_type {
    //             match self {
    //                 #name { id, .. } => id,
    //                 _ => None,
    //             }
    //         }

    //         fn set_id(&mut self, new_id: #id_type) {
    //             match self {
    //                 #name { id, .. } => *id = new_id,
    //                 _ => {},
    //             }
    //         }
    //     }
    // };

    // let expanded = quote! {
    //     impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
    //         fn id(&self) -> Option<u32> {
    //             match self {
    //                 #name { id, .. } => *id,
    //                 _ => None,
    //             }
    //         }
    
    //         fn set_id(&mut self, new_id: u32) {
    //             match self {
    //                 #name { id, .. } => *id = Some(new_id),
    //                 _ => {},
    //             }
    //         }
    //     }
    // };

    let expanded = match input.data {
        Data::Struct(data) => {
            // ...
            quote! {
                impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
                    fn id(&self) -> Option<u32> {
                        match self {
                            #name { id, .. } => *id,
                            _ => None,
                        }
                    }
            
                    fn set_id(&mut self, new_id: u32) {
                        match self {
                            #name { id, .. } => *id = Some(new_id),
                            _ => {},
                        }
                    }
                }
            }
        }
        Data::Enum(data) => {
            let variants = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");
                        if let Some(id_field) = id_field {
                            quote! {
                                #name::#variant_name { id, .. } => *id,
                            }
                        } else {
                            quote! {
                                #name::#variant_name { .. } => None,
                            }
                        }
                    }
                    _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
                }
            });
            let variants_set_id = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let id_field = fields.named.iter().find(|field| field.ident.as_ref().unwrap() == "id");
                        if let Some(id_field) = id_field {
                            quote! {
                                #name::#variant_name { id, .. } => *id = Some(new_id),
                            }
                        } else {
                            quote! {
                                #name::#variant_name { .. } => {},
                            }
                        }
                    }
                    _ => panic!("JsonRpcMessage can only be derived for enums with named fields"),
                }
            });
    
            quote! {
                impl #impl_generics JsonRpcMessage for #name #ty_generics #where_clause {
                    fn id(&self) -> Option<u32> {
                        match self {
                            #(#variants)*
                        }
                    }
    
                    fn set_id(&mut self, new_id: u32) {
                        match self {
                            #(#variants_set_id)*
                        }
                    }
                }
            }
        }
        _ => panic!("JsonRpcMessage can only be derived for structs and enums"),
    };

    TokenStream::from(expanded)
}