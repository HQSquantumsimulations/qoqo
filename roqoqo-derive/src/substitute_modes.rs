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

use crate::{extract_fields_with_types, extract_variants_with_types};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Ident};

/// Dispatch to derive SubstituteModes for enums and structs
pub fn dispatch_struct_enum(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(ds) => substitute_modes_struct(ds, ident),
        Data::Enum(de) => substitute_modes_enum(de, ident),
        _ => panic!("SubstituteModes can only be derived on structs and enums"),
    }
}

/// Create the TokenStream the SubstituteModes trait for enums
fn substitute_modes_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();

    let remap_quotes = variants_with_type.map(|(vident, _ ,_)| {
        quote! {
            &#ident::#vident(ref inner) => {Ok(#ident::#vident(SubstituteModes::remap_modes(&(*inner), mapping)?))},
        }
    });

    let bremap = quote! {
        /// Remaps the bosonic modes in clone of the operation.
        fn remap_modes(&self, mapping: &std::collections::HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
            match self{
                #(#remap_quotes)*
                _ => panic!("Unexpectedly cannot match variant")
            }
        }
    };
    quote! {
        /// Implements [SubstituteModes] trait allowing to perform bosonic mode mappings.
        #[automatically_derived]
        impl SubstituteModes for #ident{
            #bremap
        }

    }
}

/// Generate TokenStream of implementation of Operate for structs
fn substitute_modes_struct(ds: DataStruct, ident: Ident) -> TokenStream {
    let fields_with_type = extract_fields_with_types(ds).into_iter();

    let remap_quote = fields_with_type.map(|(fid, _, _)| match fid.to_string().as_str() {
        "mode" => quote! {*mapping.get(&self.mode).unwrap_or(&self.mode)},
        "mode_0" => quote! {*mapping.get(&self.mode_0).unwrap_or(&self.mode_0)},
        "mode_1" => quote! {*mapping.get(&self.mode_1).unwrap_or(&self.mode_1)},
        _ => quote! {(self).#fid.clone()},
    });
    quote! {
        /// Implements [SubstituteModes] trait allowing to perform bosonic mode mappings.
        #[automatically_derived]
        impl SubstituteModes for #ident{
            /// Remaps the modes in clone of the operation.
            fn remap_modes(&self, mapping: &std::collections::HashMap<usize, usize>) -> Result<Self, RoqoqoError>{
                crate::operations::check_valid_mapping(mapping)?;
                Ok(Self::new(#(#remap_quote),*))
            }
        }
    }
}
