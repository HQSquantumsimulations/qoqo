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

use crate::extract_variants_with_types;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Ident};

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

    let supported_version_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {SupportedVersion::minimum_supported_roqoqo_version(&(*inner))},
        }
    });
    let qversion = quote! {
        fn minimum_supported_roqoqo_version(&self) ->(u32, u32, u32) {
            match self{
                #(#supported_version_quotes)*
                _ => panic!("Unexpectedly cannot match variant")
            }
        }
    };

    quote! {
        #[automatically_derived]
        impl SupportedVersion for #ident{
            #qversion
        }
    }
}

/// Generates the TokenStream of the implementation of Operate for structs
fn operate_qubits_struct(_ds: DataStruct, ident: Ident) -> TokenStream {
    // let reserved_fields: HashSet<&str> = RESERVED_FIELDS.iter().cloned().collect();
    // let fields_with_type = extract_fields_with_types(ds).into_iter();
    quote! {
        #[automatically_derived]
        impl SupportedVersion for #ident{}

    }
}
