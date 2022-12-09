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

/// Dispatch to derive Substitute for enums and structs
pub fn dispatch_struct_enum(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(ds) => substitute_struct(ds, ident),
        Data::Enum(de) => substitute_enum(de, ident),
        _ => panic!("InvolveQubits can only be derived on structs and enums"),
    }
}

/// Create the TokenStream the Substitute trait for enums
fn substitute_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();

    let substitute_quotes = variants_with_type.clone().map(|(vident, _ ,_)| {
        quote! {
            &#ident::#vident(ref inner) => {Ok(#ident::#vident(Substitute::substitute_parameters(&(*inner), calculator)?))},
        }
    });
    let remap_quotes = variants_with_type.map(|(vident, _ ,_)| {
        quote! {
            &#ident::#vident(ref inner) => {Ok(#ident::#vident(Substitute::remap_qubits(&(*inner), mapping)?))},
        }
    });

    let qsubstitute = quote! {
        /// Substitutes symbolic parameters in clone of the operation.
        fn substitute_parameters(&self, calculator: &qoqo_calculator::Calculator) -> Result<Self, RoqoqoError> {
            match self{
                #(#substitute_quotes)*
                _ => panic!("Unexpectedly cannot match variant")
            }
        }
    };
    let qremap = quote! {
        /// Remaps the qubits in clone of the operation.
        fn remap_qubits(&self, mapping: &std::collections::HashMap<usize, usize>) -> Result<Self, RoqoqoError> {
            match self{
                #(#remap_quotes)*
                _ => panic!("Unexpectedly cannot match variant")
            }
        }
    };
    quote! {
        /// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
        #[automatically_derived]
        impl Substitute for #ident{
            #qsubstitute
            #qremap
        }
    }
}

/// Generate TokenStream of implementation of Operate for structs
fn substitute_struct(ds: DataStruct, ident: Ident) -> TokenStream {
    let fields_with_type = extract_fields_with_types(ds).into_iter();

    let substitute_quote = fields_with_type
        .clone()
        .map(|(id, type_string, _)| match type_string {
            Some(s) => match s.as_str() {
                "CalculatorFloat" => {
                    quote! {qoqo_calculator::CalculatorFloat::from(calculator.parse_get((self).#id.clone())?)}
                }
                "Circuit" => quote! {(self).#id.substitute_parameters(calculator)?},
                _ => quote! {(self).#id.clone()},
            },
            _ => quote! {(self).#id.clone()},
        });
    let mut contains_qubits = false;
    let remap_quote = fields_with_type
        .clone()
        .map(|(fid, _, _)| match fid.to_string().as_str() {
            "qubit" => quote! {*mapping.get(&self.qubit).unwrap_or(&self.qubit)},
            "control" => quote! {*mapping.get(&self.control).unwrap_or(&self.control)},
            "target" => quote! {*mapping.get(&self.target).unwrap_or(&self.target)},
            "qubits" => quote! { new_qubits },
            _ => quote! {(self).#fid.clone()},
        });
    for (fid, _, _) in fields_with_type {
        if fid.to_string().as_str() == "qubits" {
            contains_qubits = true
        }
    }
    let new_qubits_quote = if contains_qubits {
        quote! {
            let mut new_qubits: Vec<usize> = Vec::new();
            for q in &self.qubits{
                new_qubits.push(*mapping.get(q).unwrap_or(q))
            }
        }
    } else {
        quote! {}
    };
    quote! {
        /// Implements [Substitute] trait allowing to replace symbolic parameters and to perform qubit mappings.
        #[automatically_derived]
        impl Substitute for #ident{
            /// Substitutes symbolic parameters in clone of the operation.
            fn substitute_parameters(&self, calculator: &qoqo_calculator::Calculator) -> Result<Self, RoqoqoError> {
                Ok(Self::new(#(#substitute_quote),*))
            }
            /// Remaps the qubits in clone of the operation.
            fn remap_qubits(&self, mapping: &std::collections::HashMap<usize, usize>) -> Result<Self, RoqoqoError>{
                crate::operations::check_valid_mapping(mapping)?;
                #new_qubits_quote
                Ok(Self::new(#(#remap_quote),*))
            }
        }
    }
}
