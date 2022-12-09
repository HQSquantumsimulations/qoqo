// Copyright © 2021-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use crate::{extract_fields_with_types, extract_variants_with_types, RESERVED_FIELDS};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Ident, Type, TypePath};

/// Dispatch to derive Operate for enums and structs
pub fn dispatch_struct_enum(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(ds) => operate_qubits_struct(ds, ident),
        Data::Enum(de) => operate_qubits_enum(de, ident),
        _ => panic!("InvolveQubits can only be derived on structs and enums"),
    }
}

/// Creates the TokenStream of the InvolveQubits trait for enums
fn operate_qubits_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    // TokenStream iterator for matching tags function on variants
    let tags_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {Operate::tags(&(*inner))},
        }
    });
    // TokenStream iterator for matching hqslang function on variants
    let hqslang_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {Operate::hqslang(&(*inner))},
        }
    });
    // TokenStream iterator for matching From implementation on variants
    let from_quotes = variants_with_type.clone().map(|(vident, _, ty)| {
        quote! {
            /// Implements the [From] trait from this Operation.
            #[automatically_derived]
            impl From<#ty> for #ident{
                /// Performs the conversion.
                fn from(v: #ty) -> Self{
                    #ident::#vident(v)
                }
            }
        }
    });
    // TokenStream iterator for matching TryFrom implementation on variants
    let try_from_quotes = variants_with_type.clone().map(|(vident, _, ty)| {
        quote! {
            #[automatically_derived]
            /// Implements the [core::convert::TryFrom] trait into this Operation.
            impl core::convert::TryFrom<#ident> for #ty{
                /// The type returned in case of a conversion error.
                type Error = &'static str;
                /// Performs the conversion.
                fn try_from(e: #ident) -> Result<Self, Self::Error>{
                    match e{
                        #ident::#vident(v) => Ok(v),
                        _ => Err("Type can not be created from variant")
                    }

                }
            }
        }
    });
    let is_parametrized_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {Operate::is_parametrized(&(*inner))},
        }
    });
    let qtags = quote! {
        /// Returns tags classifying the type of the Operation.
        fn tags(&self) -> &'static [&'static str] {
            match self{
                #(#tags_quotes)*
                _ => panic!("Unexpectedly cannot match variant")
            }
        }
    };
    let qhqslang = quote! {
        /// Returns hqslang name of the Operation.
        fn hqslang(&self) -> &'static str {
            match self{
                #(#hqslang_quotes)*
                _ => panic!("Unexpectedly cannot match variant")
            }
        }
    };
    let qisparametrized = quote! {
        /// Returns `true` when Operation has symbolic parameters.
        fn is_parametrized(&self) -> bool {
            match self{
                #(#is_parametrized_quotes)*
                _ => panic!("Unexpectedly cannot match variant")
            }
        }
    };
    quote! {
        #[automatically_derived]
        #[cfg_attr(feature = "dynamic", typetag::serde)]
        /// Implements the universal basic trait [Operate] for this Operation.
        impl Operate for #ident{
            #qtags
            #qhqslang
            #qisparametrized
        }
        #(#from_quotes)*
        #(#try_from_quotes)*
    }
}

/// Generates the TokenStream of the implementation of Operate for structs
fn operate_qubits_struct(ds: DataStruct, ident: Ident) -> TokenStream {
    let reserved_fields: HashSet<&str> = RESERVED_FIELDS.iter().cloned().collect();
    let fields_with_type = extract_fields_with_types(ds).into_iter();

    let input_arguments = fields_with_type.clone().map(|(id, _, ty)| {
        quote! {#id: #ty}
    });

    let arguments = fields_with_type.clone().map(|(id, _, _)| {
        quote! {#id}
    });

    let calculator_float_fields = fields_with_type
        .clone()
        .filter(|(_, _, ty)| match &ty {
            Type::Path(TypePath { path: p, .. }) => match p.get_ident() {
                None => false,
                Some(id) => *id == "CalculatorFloat",
            },
            _ => false,
        })
        .map(|(id, _, _)| {
            quote! {
                !self.#id.is_float()
            }
        });
    let circuit_fields = fields_with_type
        .clone()
        .filter(|(_, type_string, _)| type_string == &Some("Circuit".to_string()))
        .map(|(id, _, _)| {
            quote! {
                self.#id.is_parametrized()
            }
        });

    let circuit_fields2 = fields_with_type
        .clone()
        .filter(|(_, type_string, _)| type_string == &Some("Option<Circuit>".to_string()))
        .map(|(id, _, _)| {
            quote! {
                match self.#id.as_ref(){
                    Some(x) => x.is_parametrized(),
                    None => false
                }
            }
        });
    let is_parametrized_fields = if calculator_float_fields.clone().last().is_none()
        && circuit_fields.clone().last().is_none()
        && circuit_fields2.clone().last().is_none()
    {
        vec![quote!(false)]
    } else {
        calculator_float_fields
            .chain(circuit_fields)
            .chain(circuit_fields2)
            .collect()
    };
    let getter_fields = fields_with_type
        .filter(|(id, _, _)| {
            // let id = &field.ident.clone().expect("Struct fields must be named");
            !reserved_fields.contains(id.to_string().as_str())
        })
        .map(|(id, _, ty)| {
            let msg = format!("Returns the value of the field `{}`.", id);
            quote! {
                #[doc = #msg]
                #[inline]
                pub fn #id(&self) -> &#ty{
                    &self.#id
                }
            }
        });
    let formated_tags = format_ident!("TAGS_{}", ident);
    let formated_hqslang = format!("{}", ident);
    let msg = format!("Creates a new instance of `{}`.\n\n", ident);
    quote! {
        #[automatically_derived]
        impl #ident{
            #(#getter_fields)*

            #[doc = #msg]
            #[inline]
            pub fn new(#(#input_arguments),*) -> Self{
                Self{#(#arguments),*}
            }
        }

        #[automatically_derived]
        #[cfg_attr(feature = "dynamic", typetag::serde)]
        /// Implements the universal basic trait [Operate] for this Operation.
        impl Operate for #ident{
            /// Returns `true` when Operation has symbolic parameters.
            #[inline]
            fn is_parametrized(&self) -> bool {
                    // Leading false is necessary for constant operations that are never parametrized
                    (#(#is_parametrized_fields)&&*)
            }
            /// Returns tags classifying the type of the Operation.
            #[inline]
            fn tags(&self) -> &'static [&'static str]{
                #formated_tags
            }
            /// Returns hqslang name of the Operation.
            #[inline]
            fn hqslang(&self) -> &'static str{
                #formated_hqslang
            }
        }

    }
}

/// Dispatch to derive Operation TryFrom implementations for enums
pub fn dispatch_try_from_enum(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Enum(de) => try_from_operations_enum(de, ident),
        _ => panic!("TryFrom implementations for Operation can only be derived on enums"),
    }
}

/// Create the TokenStream that implements TryFrom for Operation
fn try_from_operations_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let id_name = ident.to_string();
    let variants_with_type = extract_variants_with_types(de).into_iter();

    let operation_from_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            #ident::#vident(inner) => {Operation::#vident(inner)}
        }
    });
    let from_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            Operation::#vident(inner) => {Ok(#ident::#vident(inner))}
        }
    });
    let from_clone_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            Operation::#vident(inner) => {Ok(#ident::#vident(inner.clone()))}
        }
    });
    quote! {impl  std::convert::From<#ident> for Operation{
        fn from(op: #ident) -> Self {
            match op{
            #(#operation_from_quotes),*
            }
        }
    }

    impl  std::convert::TryFrom<Operation> for #ident{
        type Error = RoqoqoError;
        fn try_from(op: Operation) -> Result<Self, Self::Error> {
            match op{
            #(#from_quotes),*
            _ => Err(RoqoqoError::ConversionError{start_type:"Operation", end_type:#id_name}),
            }
        }
    }

    impl  std::convert::TryFrom<&Operation> for #ident{
        type Error = RoqoqoError;
        fn try_from(op: &Operation) -> Result<Self, Self::Error> {
            match op{
            #(#from_clone_quotes),*
            _ => Err(RoqoqoError::ConversionError{start_type:"Operation", end_type:#id_name}),
            }
        }
    }}
}
