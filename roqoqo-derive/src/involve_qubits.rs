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

use crate::{extract_fields_with_types, extract_variants_with_types};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Ident};

/// Dispatch to derive InvolveQubits for enums and structs
pub fn dispatch_struct_enum(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(ds) => involve_qubits_struct(ds, ident),
        Data::Enum(de) => involve_qubits_enum(de, ident),
        _ => panic!("InvolveQubits can only be derived on structs and enums"),
    }
}

/// Create the TokenStream of the InvolveQubits trait for enums
/// This derive delegates the invocations of the involved_qubits function to all possible variants via match arms
fn involve_qubits_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {InvolveQubits::involved_qubits(&(*inner))},
        }
    });

    let match_quotes_classical = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {InvolveQubits::involved_classical(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        /// Implements [InvolveQubits] trait for the qubits involved in this Operation.
        impl InvolveQubits for #ident{
            fn involved_qubits(&self) -> InvolvedQubits {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }

            fn involved_classical(&self) -> InvolvedClassical {
                match self{
                    #(#match_quotes_classical)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

/// Generate the TokenStream of the implementation of InvolvedQubits for structs
fn involve_qubits_struct(ds: DataStruct, ident: Ident) -> TokenStream {
    // We only allow structs with named fields
    // Extract named fields with match and panic if fields are not named
    let fields_with_type = extract_fields_with_types(ds).into_iter();

    // Bool values that show if there is a qubit field, control field etc. in the struct
    let mut qubit: bool = false;
    let mut control: bool = false;
    let mut target: bool = false;
    let mut qubits: bool = false;

    // Iterating over the fields in the struct and setting the bool values to true if the field is
    // in the struct
    for (fid, type_string, _) in fields_with_type {
        // Matching the name to see if we need to set one to true
        match fid.clone().to_string().as_str() {
            "qubit" => {
                if type_string == Some("usize".to_string()) {
                    qubit = true
                } else {
                    panic!("Field  qubit must have type usize")
                }
            }
            "target" => {
                if type_string == Some("usize".to_string()) {
                    target = true;
                } else {
                    panic!("Field target must have type usize")
                }
            }
            "control" => {
                if type_string == Some("usize".to_string()) {
                    control = true;
                } else {
                    panic!("Field control must have type usize")
                }
            }
            "qubits" => {
                qubits = true;
            }
            _ => {}
        };
    }
    if qubit {
        if control || target || qubits {
            panic!("When deriving InvolveQubits, qubit field is not compatible with control, target or qubits fields");
        };
        // Creating a function that puts exactly one qubit `qubit` into the InvolvedQubits HashSet
        quote! {
            /// Implements [InvolveQubits] trait for the qubits involved in this Operation.
            #[automatically_derived]
            impl InvolveQubits for #ident{
                /// Returns a list of all involved qubits.
                fn involved_qubits(&self ) -> InvolvedQubits {
                    let mut new_hash_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
                    new_hash_set.insert(self.qubit);
                    InvolvedQubits::Set(new_hash_set)
                }
            }
        }
    } else if target || control {
        if !(control && target) {
            panic!("When deriving InvolveQubits control and target fields have to both be present");
        };
        if qubits {
            panic!("When deriving InvolveQubits, control and target fields are not compatible with qubits fields");
        };
        // Creating a function that puts qubits `control` and `target` into the InvolvedQubits HashSet
        quote! {
            /// Implements [InvolveQubits] trait for the qubits involved in this Operation.
            #[automatically_derived]
            impl InvolveQubits for #ident{
                /// Returns a list of all involved qubits.
                fn involved_qubits(&self ) -> InvolvedQubits {
                    let mut new_hash_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
                    new_hash_set.insert(self.control);
                    new_hash_set.insert(self.target);
                    InvolvedQubits::Set(new_hash_set)
                }
            }
        }
    } else if qubits {
        // Creating a function that puts all qubits in the vector `qubits` into the InvolvedQubits HashSet
        quote! {
            /// Implements [InvolveQubits] trait for the qubits involved in this Operation.
            #[automatically_derived]
            impl InvolveQubits for #ident{
                /// Returns a list of all involved qubits.
                fn involved_qubits(&self ) -> InvolvedQubits {
                    let mut new_hash_set: std::collections::HashSet<usize> = std::collections::HashSet::new();
                    for qubit in self.qubits.iter(){
                        new_hash_set.insert(*qubit);
                    }
                    InvolvedQubits::Set(new_hash_set)
                }
            }
        }
    } else {
        panic!("To derive InvolveQubits qubit or control or target or qubits fields need to be present in struct")
    }
}
