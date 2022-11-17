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

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields, Ident};

/// Dispatch to derive OperateSingleQubit for enums and structs
pub fn dispatch_struct_enum_single_qubit(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_single_qubit_struct(ident),
        Data::Enum(de) => operate_single_qubit_enum(de, ident),
        _ => panic!("OperateSingleQubit can only be derived on structs and enums"),
    }
}

fn operate_single_qubit_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let DataEnum { variants, .. } = de;
    let match_quotes = variants.into_iter().map(|v| {
        // vident is the name of the variant
        let vident = v.ident.clone();
        // fields are the fields in one variant
        // Here we are very restrictive, we only want to allow enums of the form
        // enum SingleQubitGate{
        //      RotateZ(RotateZ),
        //      RotateX(RotateX)
        // }
        // where the variants are so called newtype structs (structs with exactly one unnamed field)
        // This kind of enum is enough to represent our enum_dispatch-like structure of enums collecting different Operations

        // We match the fields in the variant,
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            // and panic when the fields are not unnamed,
            _ => panic!(
                "OperateSingleQubit can only be derived for enums with newtype structs as variants"
            ),
        };
        // and panic when there is more than one field, ensuring newtype structs.
        if fields.unnamed.iter().len() != 1 {
            panic!(
                "OperateSingleQubit can only be derived for enums with newtype structs as variants"
            )
        }
        quote! {
            &#ident::#vident(ref inner) => {OperateSingleQubit::qubit(&(*inner))},
        }
    });
    quote! {
        /// Implements [OperateSingleQubit] trait for this Operation acting on exactly one qubit.
        #[automatically_derived]
        impl OperateSingleQubit for #ident{
            /// Returns `qubit` the Operation acts on.
            fn qubit(&self) -> &usize {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

/// Generate TokenStream of implementation of OperateSingleQubit for structs
fn operate_single_qubit_struct(ident: Ident) -> TokenStream {
    quote! {
        /// Implements [OperateSingleQubit] trait for this Operation acting on exactly one qubit.
        #[automatically_derived]
        impl OperateSingleQubit for #ident{
            /// Returns `qubit` the Operation acts on.
            fn qubit(&self ) -> &usize {
                &self.qubit
            }
        }

    }
}

/// Dispatch to derive OperateTwoQubit for enums and structs
pub fn dispatch_struct_enum_two_qubit(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_two_qubit_struct(ident),
        Data::Enum(de) => operate_two_qubit_enum(de, ident),
        // There can be other objects that are valid Derive inputs, but we define our macro only for structs and enums
        _ => panic!("OperateTwoQubit can only be derived on structs and enums"),
    }
}

fn operate_two_qubit_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let DataEnum { variants, .. } = de;
    let control_quotes = variants.clone().into_iter().map(|v| {
        let vident = v.ident.clone();

        // We match the fields in the variant,
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            // and panic when the fields are not unnamed,
            _ => panic!(
                "OperateSingleQubit can only be derived for enums with newtype structs as variants"
            ),
        };
        // and panic when there is more than one field, ensuring newtype structs.
        if fields.unnamed.iter().len() != 1 {
            panic!(
                "OperateSingleQubit can only be derived for enums with newtype structs as variants"
            )
        }
        quote! {
            &#ident::#vident(ref inner) => {OperateTwoQubit::control(&(*inner))},
        }
    });
    let target_quotes = variants.into_iter().map(|v| {
        let vident = v.ident.clone();
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            _ => panic!(
                "OperateSingleQubit can only be derived for enums with newtype structs as variants"
            ),
        };
        if fields.unnamed.iter().len() != 1 {
            panic!(
                "OperateSingleQubit can only be derived for enums with newtype structs as variants"
            )
        }
        quote! {
            &#ident::#vident(ref inner) => {OperateTwoQubit::target(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        /// Trait for Operations acting on exactly two qubits.
        impl OperateTwoQubit for #ident{
            /// Returns `control` qubit of the two qubit Operation.
            fn control(&self) -> &usize {
                match self{
                    #(#control_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            /// Returns `target` qubit of the two qubit Operation.
            fn target(&self) -> &usize {
                match self{
                    #(#target_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

/// Generate TokenStream of implementation of OperateSingleQubit for structs
fn operate_two_qubit_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        /// Trait for Operations acting on exactly two qubits.
        impl OperateTwoQubit for #ident{
            /// Returns `control` qubit of the two qubit Operation.
            fn control(&self ) -> &usize {
                &self.control
            }
            /// Returns `target` qubit of the two qubit Operation.
            fn target(&self ) -> &usize {
                &self.target
            }
        }

    }
}

pub fn dispatch_struct_enum_multi_qubit(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_multi_qubit_struct(ident),
        Data::Enum(de) => operate_multi_qubit_enum(de, ident),
        _ => panic!("OperateSingleQubit can only be derived on structs and enums"),
    }
}

fn operate_multi_qubit_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let DataEnum { variants, .. } = de;
    let match_quotes = variants.into_iter().map(|v| {
        let vident = v.ident.clone();
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            _ => panic!(
                "OperateMultiQubit can only be derived for enums with newtype structs as variants"
            ),
        };
        if fields.unnamed.iter().len() != 1 {
            panic!(
                "OperateMultiQubit can only be derived for enums with newtype structs as variants"
            )
        }
        quote! {
            &#ident::#vident(ref inner) => {OperateMultiQubit::qubits(&(*inner))},
        }
    });
    quote! {
        /// Implements [OperateMultiQubit] trait for this Operation acting on a set of qubits.
        #[automatically_derived]
        impl OperateMultiQubit for #ident{
            /// Returns `qubit` the Operation acts on.
            fn qubits(&self) -> &Vec<usize> {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

fn operate_multi_qubit_struct(ident: Ident) -> TokenStream {
    quote! {
        /// Implements [OperateMultiQubit] trait for this Operation acting on a set of qubits.
        #[automatically_derived]
        impl OperateMultiQubit for #ident{
            /// Returns `qubit` the Operation acts on.
            fn qubits(&self ) -> &Vec<usize> {
                &self.qubits
            }
        }
    }
}
