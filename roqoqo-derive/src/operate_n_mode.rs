// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields, Ident};

/// Dispatch to derive OperateSingleMode for enums and structs
pub fn dispatch_struct_enum_single_mode(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_single_mode_struct(ident),
        Data::Enum(de) => operate_single_mode_enum(de, ident),
        _ => panic!("OperateSingleMode can only be derived on structs and enums"),
    }
}

fn operate_single_mode_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let DataEnum { variants, .. } = de;
    let match_quotes = variants.into_iter().map(|v| {
        // vident is the name of the variant
        let vident = v.ident.clone();
        // fields are the fields in one variant
        // Here we are very restrictive, we only want to allow enums of the form
        // enum SingleModeGate{
        //      Squeezing(Squeezing),
        // }
        // where the variants are so called newtype structs (structs with exactly one unnamed field)
        // This kind of enum is enough to represent our enum_dispatch-like structure of enums collecting different Operations

        // We match the fields in the variant,
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            // and panic when the fields are not unnamed,
            _ => panic!(
                "OperateSingleMode can only be derived for enums with newtype structs as variants"
            ),
        };
        // and panic when there is more than one field, ensuring newtype structs.
        if fields.unnamed.iter().len() != 1 {
            panic!(
                "OperateSingleMode can only be derived for enums with newtype structs as variants"
            )
        }
        quote! {
            &#ident::#vident(ref inner) => {OperateSingleMode::mode(&(*inner))},
        }
    });
    quote! {
        /// Implements [OperateSingleMode] trait for this Operation acting on exactly one bosonic mode.
        #[automatically_derived]
        impl OperateSingleMode for #ident{
            /// Returns bosonic `mode` the Operation acts on.
            fn mode(&self) -> &usize {
                match self {
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

/// Generate TokenStream of implementation of OperateSingleMode for structs
fn operate_single_mode_struct(ident: Ident) -> TokenStream {
    quote! {
        /// Implements [OperateSingleMode] trait for this Operation acting on exactly one bosonic mode.
        #[automatically_derived]
        impl OperateSingleMode for #ident{
            /// Returns bosonic `mode` the Operation acts on.
            fn mode(&self) -> &usize {
                &self.mode
            }
        }

    }
}

/// Dispatch to derive OperateTwoMode for enums and structs
pub fn dispatch_struct_enum_two_mode(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_two_mode_struct(ident),
        Data::Enum(de) => operate_two_mode_enum(de, ident),
        // There can be other objects that are valid Derive inputs, but we define our macro only for structs and enums
        _ => panic!("OperateTwoMode can only be derived on structs and enums"),
    }
}

fn operate_two_mode_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let DataEnum { variants, .. } = de;
    let mode_0_quotes = variants.clone().into_iter().map(|v| {
        let vident = v.ident.clone();

        // We match the fields in the variant,
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            // and panic when the fields are not unnamed,
            _ => panic!(
                "OperateSingleMode can only be derived for enums with newtype structs as variants"
            ),
        };
        // and panic when there is more than one field, ensuring newtype structs.
        if fields.unnamed.iter().len() != 1 {
            panic!(
                "OperateSingleMode can only be derived for enums with newtype structs as variants"
            )
        }
        quote! {
            &#ident::#vident(ref inner) => {OperateTwoMode::mode_0(&(*inner))},
        }
    });
    let mode_1_quotes = variants.into_iter().map(|v| {
        let vident = v.ident.clone();
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            _ => panic!(
                "OperateSingleMode can only be derived for enums with newtype structs as variants"
            ),
        };
        if fields.unnamed.iter().len() != 1 {
            panic!(
                "OperateSingleMode can only be derived for enums with newtype structs as variants"
            )
        }
        quote! {
            &#ident::#vident(ref inner) => {OperateTwoMode::mode_1(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        /// Trait for Operations acting on exactly two bosonic modes.
        impl OperateTwoMode for #ident{
            /// Returns bosonic mode `mode_0` of the two-bosonic mode Operation.
            fn mode_0(&self) -> &usize {
                match self{
                    #(#mode_0_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            /// Returns bosonic mode `mode_1` of the two-bosonic mode Operation.
            fn mode_1(&self) -> &usize {
                match self{
                    #(#mode_1_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

/// Generate TokenStream of implementation of OperateSingleMode for structs
fn operate_two_mode_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        /// Trait for Operations acting on exactly two bosonic modes.
        impl OperateTwoMode for #ident{
            /// Returns bosonic mode `mode_0` of the two-bosonic mode Operation.
            fn mode_0(&self ) -> &usize {
                &self.mode_0
            }
            /// Returns bosonic mode `mode_1` of the two-bosonic mode Operation.
            fn mode_1(&self) -> &usize {
                &self.mode_1
            }
        }

    }
}
