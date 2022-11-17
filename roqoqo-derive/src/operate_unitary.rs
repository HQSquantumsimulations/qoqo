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
use syn::{Data, DataEnum, DeriveInput, Ident};

pub fn dispatch_struct_enum_operate_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_gate_struct(ident),
        Data::Enum(de) => operate_gate_enum(de, ident),
        _ => panic!("OperateGate can only be derived on enums"),
    }
}

fn operate_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateGate::unitary_matrix(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        impl OperateGate for #ident{
            fn unitary_matrix(&self) -> Result<Array2<Complex64>, RoqoqoError> {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

fn operate_gate_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperateGate for #ident{
            fn unitary_matrix(&self ) -> Result<Array2<Complex64>, RoqoqoError> {
                self.unitary_matrix
            }
        }
    }
}

/// Dispatch to derive OperateSingleQubitGate for enums and structs
pub fn dispatch_struct_enum_single_qubit_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => single_qubit_gate_struct(ident),
        Data::Enum(de) => single_qubit_gate_enum(de, ident),
        _ => panic!("OperateSingleQubit can only be derived on structs and enums"),
    }
}

fn single_qubit_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let alpha_r_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateSingleQubitGate::alpha_r(&(*inner))},
        }
    });
    let beta_i_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateSingleQubitGate::beta_i(&(*inner))},
        }
    });
    let alpha_i_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateSingleQubitGate::alpha_i(&(*inner))},
        }
    });
    let beta_r_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateSingleQubitGate::beta_r(&(*inner))},
        }
    });
    let phase_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateSingleQubitGate::global_phase(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        impl OperateSingleQubitGate for #ident{
            fn alpha_r(&self) -> CalculatorFloat {
                match self{
                    #(#alpha_r_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            fn alpha_i(&self) -> CalculatorFloat {
                match self{
                    #(#alpha_i_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            fn beta_r(&self) -> CalculatorFloat {
                match self{
                    #(#beta_r_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            fn beta_i(&self) -> CalculatorFloat {
                match self{
                    #(#beta_i_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            fn global_phase(&self) -> CalculatorFloat {
                match self{
                    #(#phase_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }

    }
}

fn single_qubit_gate_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl Define for #ident{
            fn name(&self ) -> &String {
                &self.name
            }
        }
        #[automatically_derived]
        impl OperateSingleQubitGate for #ident{
            fn alpha_r(&self) -> CalculatorFloat {
                self.alpha_r
            }
            fn alpha_i(&self) -> CalculatorFloat {
                self.alpha_i
            }
            fn beta_r(&self) -> CalculatorFloat {
                self.beta_r
            }
            fn beta_i(&self) -> CalculatorFloat {
                self.beta_i
            }
            fn global_phase(&self) -> CalculatorFloat {
                self.global_phase
            }
        }
    }
}

pub fn dispatch_struct_enum_constant_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Enum(de) => constant_gate_enum(de, ident),
        _ => panic!("OperateConstantGate can only be derived on  enums"),
    }
}

fn constant_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => { OperateConstantGate::inverse(&(*inner)).into()},
        }
    });
    quote! {
        #[automatically_derived]
        impl OperateConstantGate for #ident{
            fn inverse(&self) -> GateOperation {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

pub fn dispatch_struct_enum_two_qubit_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => two_qubit_gate_struct(ident),
        Data::Enum(de) => two_qubit_gate_enum(de, ident),
        _ => panic!("OperateTwoQubitGate can only be derived on structs and enums"),
    }
}

fn two_qubit_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateTwoQubitGate::kak_decomposition(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        impl OperateTwoQubitGate for #ident{
            fn kak_decomposition(&self) -> KakDecomposition {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

fn two_qubit_gate_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperateTwoQubitGate for #ident{
            fn kak_decomposition(&self ) -> KakDecomposition {
                self.kak_decomposition
            }
        }
    }
}

pub fn dispatch_struct_enum_multi_qubit_gate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => multi_qubit_gate_struct(ident),
        Data::Enum(de) => multi_qubit_gate_enum(de, ident),
        _ => panic!("OperateMultiQubitGate can only be derived on structs and enums"),
    }
}

fn multi_qubit_gate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperateMultiQubitGate::circuit(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        impl OperateMultiQubitGate for #ident{
            fn circuit(&self) -> crate::Circuit {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

fn multi_qubit_gate_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperateMultiQubitGate for #ident{
            fn circuit(&self ) -> crate::Circuit {
                self.circuit
            }
        }
    }
}

pub fn dispatch_struct_enum_rotate(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => rotate_struct(ident),
        Data::Enum(de) => rotate_enum(de, ident),
        _ => panic!("OperateSingleQubit can only be derived on structs and enums"),
    }
}

fn rotate_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {Rotate::theta(&(*inner))},
        }
    });
    let overrotate_quote = if cfg!(feature = "overrotate") {
        let overrotate_match_quotes = variants_with_type.clone().map(|(vident, _, _)|  {
            quote! {
                &#ident::#vident(ref inner) => {#ident::#vident(Rotate::overrotate(&(*inner), amplitude, variance))}
            }
        });
        quote! {
                fn overrotate(&self, amplitude: &f64, variance: &f64) -> Self {
                    match self{
                        #(#overrotate_match_quotes),*
                        _ => panic!("Unexpectedly cannot match variant"),
                    }
                }
        }
    } else {
        quote! {}
    };
    let match_pow_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => #ident::#vident(Rotate::powercf(&(*inner), power)),
        }
    });

    quote! {
        #[automatically_derived]
        impl Rotate for #ident{
            fn theta(&self) -> &CalculatorFloat {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            fn powercf(&self, power: CalculatorFloat) -> #ident {
                match self{
                    #(#match_pow_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            #overrotate_quote
        }

    }
}

fn rotate_struct(ident: Ident) -> TokenStream {
    let overrotate_quote = if cfg!(feature = "overrotate") {
        quote! {
            fn overrotate(&self, amplitude: &f64, variance: &f64) -> Self {
                let mut return_gate = self.clone();
                let distr = Normal::new(0.0, *variance).unwrap();
                return_gate.theta += *amplitude * distr.sample(&mut rand::thread_rng());
                return_gate
            }

        }
    } else {
        quote! {}
    };
    quote! {
        #[automatically_derived]
        impl Rotate for #ident{
            fn theta(&self ) -> &CalculatorFloat {
                &self.theta
            }
            fn powercf(&self, power: CalculatorFloat) -> #ident {
                let mut new = self.clone();
                new.theta = power * self.theta.clone();
                new
            }
            #overrotate_quote
        }
    }
}

pub fn dispatch_struct_enum_define(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => define_struct(ident),
        Data::Enum(de) => define_enum(de, ident),
        _ => panic!("Definition can only be derived on structs and enums"),
    }
}

fn define_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {Define::name(&(*inner))},
        }
    });
    quote! {
        #[automatically_derived]
        impl Define for #ident{
            fn name(&self) -> &String {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    }
}

fn define_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl Define for #ident{
            fn name(&self ) -> &String {
                &self.name
            }
        }
    }
}

pub fn dispatch_struct_enum_operate_pragma(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(_ds) => operate_pragma_struct(ident),
        Data::Enum(de) => operate_pragma_enum(de, ident),
        _ => panic!("OperatePragma can only be derived on structs and enums"),
    }
}

fn operate_pragma_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let _match_quotes = variants_with_type.map(|(_vident, _, _)| {
        quote! {}
    });
    quote! {
        #[automatically_derived]
        impl OperatePragma for #ident{
        }
    }
}

fn operate_pragma_struct(ident: Ident) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl OperatePragma for #ident{
        }
    }
}

pub fn dispatch_struct_enum_operate_noise_pragma(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Enum(de) => operate_noise_pragma_enum(de, ident),
        _ => panic!("OperatePragmaNoise can only be derived on enums"),
    }
}

fn operate_noise_pragma_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_quotes = variants_with_type.clone().map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => {OperatePragmaNoise::superoperator(&(*inner))},
        }
    });
    let match_pow_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => #ident::#vident(OperatePragmaNoise::powercf(&(*inner), power)),
        }
    });
    let q = quote! {
        #[automatically_derived]
        impl OperatePragmaNoise for #ident{
            fn superoperator(&self) -> Result<Array2<f64>, RoqoqoError> {
                match self{
                    #(#match_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
            fn powercf(&self, power: CalculatorFloat) -> #ident {
                match self{
                    #(#match_pow_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    };
    q
}

pub fn dispatch_struct_enum_operate_noise_proba_pragma(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Enum(de) => operate_noise_proba_pragma_enum(de, ident),
        _ => panic!("OperatePragmaNoiseProba can only be derived on enums"),
    }
}

fn operate_noise_proba_pragma_enum(de: DataEnum, ident: Ident) -> TokenStream {
    let variants_with_type = extract_variants_with_types(de).into_iter();
    let match_proba_quotes = variants_with_type.map(|(vident, _, _)| {
        quote! {
            &#ident::#vident(ref inner) => inner.probability(),
        }
    });
    let q = quote! {
        #[automatically_derived]
        impl OperatePragmaNoiseProba for #ident{
            fn probability(&self) -> CalculatorFloat{
                match self{
                    #(#match_proba_quotes)*
                    _ => panic!("Unexpectedly cannot match variant")
                }
            }
        }
    };
    q
}
