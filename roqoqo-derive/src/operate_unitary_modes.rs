// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use crate::extract_variants_with_types;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Ident};

pub fn dispatch_struct_enum_operate_mode_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_mode_gate_struct(ident),
        Data::Enum(de) => operate_mode_gate_enum(de, ident),
        _ => panic!("OperateModeGate can only be derived on structs and enums"),
    }
}

fn operate_mode_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let _variants_with_type = extract_variants_with_types(de).into_iter();
    quote! {
        #[automatically_derived]
        impl OperateModeGate for #ident{
        }
    }
}

fn operate_mode_gate_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperateModeGate for #ident{
        }
    }
}

/// Dispatch to derive OperateSingleModeGate for enums and structs
pub fn dispatch_struct_enum_single_mode_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => single_mode_gate_struct(ident),
        Data::Enum(de) => single_mode_gate_enum(de, ident),
        _ => panic!("OperateSingleMode can only be derived on structs and enums"),
    }
}

fn single_mode_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let _variants_with_type = extract_variants_with_types(de).into_iter();
    quote! {
        #[automatically_derived]
        impl OperateSingleModeGate for #ident{
        }

    }
}

fn single_mode_gate_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperateSingleModeGate for #ident{
        }
    }
}

pub fn dispatch_struct_enum_two_mode_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => two_mode_gate_struct(ident),
        Data::Enum(de) => two_mode_gate_enum(de, ident),
        _ => panic!("OperateTwoModeGate can only be derived on structs and enums"),
    }
}

fn two_mode_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let _variants_with_type = extract_variants_with_types(de).into_iter();
    quote! {
        #[automatically_derived]
        impl OperateTwoModeGate for #ident{
        }
    }
}

fn two_mode_gate_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperateTwoModeGate for #ident{
        }
    }
}
