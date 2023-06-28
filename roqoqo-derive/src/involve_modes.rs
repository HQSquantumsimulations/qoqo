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

use crate::{extract_fields_with_types, extract_variants_with_types};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Ident};

/// Dispatch to derive InvolveModes for enums and structs
pub fn dispatch_struct_enum(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(ds) => involve_modes_struct(ds, ident),
        Data::Enum(de) => involve_modes_enum(de, ident),
        _ => panic!("InvolveModes can only be derived on structs and enums"),
    }
}

/// Create the TokenStream of the InvolveModes trait for enums
/// This derive delegates the invocations of the involved_modes function to all possible variants via match arms
fn involve_modes_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {InvolveModes::involved_modes(&(*inner))},
        }
    });

    quote! {
        #[automatically_derived]
        /// Implements [InvolveModes] trait for the modes involved in this Operation.
        impl InvolveModes for #ident{
            fn involved_modes(&self) -> InvolvedModes {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

/// Generate the TokenStream of the implementation of InvolveModes for structs
fn involve_modes_struct(ds: DataStruct, ident: Ident) -> TokenStream {
    // We only allow structs with named fields
    // Extract named fields with match and panic if fields are not named
    let fields_with_type = extract_fields_with_types(ds).into_iter();

    // Bool values that show if there is a mode field, mode_0 field etc. in the struct
    let mut mode: bool = false;
    let mut mode_0: bool = false;
    let mut mode_1: bool = false;

    // Iterating over the fields in the struct and setting the bool values to true if the field is
    // in the struct
    for (fid, type_string, _) in fields_with_type {
        // Matching the name to see if we need to set one to true
        match fid.clone().to_string().as_str() {
            "mode" => {
                if type_string == Some("usize".to_string()) {
                    mode = true
                } else {
                    panic!("Field mode must have type usize")
                }
            }
            "mode_0" => {
                if type_string == Some("usize".to_string()) {
                    mode_0 = true;
                } else {
                    panic!("Field mode_0 must have type usize")
                }
            }
            "mode_1" => {
                if type_string == Some("usize".to_string()) {
                    mode_1 = true;
                } else {
                    panic!("Field mode_1 must have type usize")
                }
            }
            _ => {}
        };
    }
    if mode {
        if mode_0 || mode_1 {
            panic!("When deriving InvolveModes, mode field is not compatible with mode_0 or mode_1 fields");
        };
        // Creating a function that puts exactly one bosonic mode `mode` into the InvolvedModes HashSet
        quote! {
            /// Implements [InvolveModes] trait for the bosonic modes involved in this Operation.
            #[automatically_derived]
            impl InvolveModes for #ident{
                /// Returns a list of all involved bosonic modes.
                fn involved_modes(&self) -> InvolvedModes {
                    let mut new_hash_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
                    new_hash_set.insert(self.mode);
                    InvolvedModes::Set(new_hash_set)
                }
            }
        }
    } else if mode_0 || mode_1 {
        if !(mode_0 && mode_1) {
            panic!("When deriving InvolveModes mode_0 and mode_1 fields have to both be present");
        };
        // Creating a function that puts bosonic modes `mode_0` and `mode_1` into the InvolvedModes HashSet
        quote! {
            /// Implements [InvolveModes] trait for the bosonic modes involved in this Operation.
            #[automatically_derived]
            impl InvolveModes for #ident{
                /// Returns a list of all involved bosonic modes.
                fn involved_modes(&self) -> InvolvedModes {
                    let mut new_hash_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
                    new_hash_set.insert(self.mode_0);
                    new_hash_set.insert(self.mode_1);
                    InvolvedModes::Set(new_hash_set)
                }
            }
        }
    } else {
        panic!("To derive InvolveModes mode, mode_0 or mode_1 fields need to be present in struct")
    }
}
