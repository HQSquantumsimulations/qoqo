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

use crate::extract_variants_with_types;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Ident};

pub fn dispatch_struct_enum_operate_spins_analog(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_spins_analog_struct(ident),
        Data::Enum(de) => operate_spins_analog_enum(de, ident),
        _ => panic!("OperateSpinsAnalog can only be derived on structs and enums"),
    }
}

fn operate_spins_analog_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let _variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = _variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateSpinsAnalog::spin(&(*inner))},
        }
    });

    quote! {
        #[automatically_derived]
        impl OperateSpinsAnalog for #ident{
            fn spin(&self) -> Vec<usize> {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

fn operate_spins_analog_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperateSpinsAnalog for #ident{
            fn spin(&self) -> usize {
                self.spin
            }
        }
    }
}
