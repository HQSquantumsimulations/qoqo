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

//! Roqoqo-derive
//!
//! Derive proc-macros for the traits of roqoqo [roqoqo].

use syn::{
    parse_macro_input, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument, Ident,
    PathArguments, Type, TypePath,
};
mod involve_qubits;
mod operate;
mod operate_n_qubit;
mod operate_unitary;
mod substitute;
mod supported_version;

/// Array of field names that are reserved for use with specific traits
const RESERVED_FIELDS: &[&str; 11] = &[
    "qubit",
    "control",
    "target",
    "theta",
    "qubits",
    "global_phase",
    "alpha_r",
    "alpha_i",
    "beta_r",
    "beta_i",
    "name",
];

/// Derive macro for the InvolveQubits trait
#[proc_macro_derive(InvolveQubits)]
pub fn derive_involve_qubits(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    involve_qubits::dispatch_struct_enum(parsed_input).into()
}

/// Derive macro for the [roqoqo::Operate] trait
#[proc_macro_derive(Operate)]
pub fn derive_operate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate::dispatch_struct_enum(parsed_input).into()
}

/// Derive macro for automatically implementing TryFrom for Operation enum
#[proc_macro_derive(OperateTryFromEnum)]
pub fn derive_operate_try_from_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate::dispatch_try_from_enum(parsed_input).into()
}

/// Derive macro for the [roqoqo::Substitute] trait
#[proc_macro_derive(Substitute)]
pub fn derive_substitute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    substitute::dispatch_struct_enum(parsed_input).into()
}

/// Derive macro for the [roqoqo::SupportedVersion] trait
#[proc_macro_derive(SupportedVersion)]
pub fn derive_supported_version(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    supported_version::dispatch_struct_enum(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateSingleQubit] trait
#[proc_macro_derive(OperateSingleQubit)]
pub fn derive_operate_single_qubit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_n_qubit::dispatch_struct_enum_single_qubit(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateTwoQubit] trait
#[proc_macro_derive(OperateTwoQubit)]
pub fn derive_operate_two_qubit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_n_qubit::dispatch_struct_enum_two_qubit(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateMultiQubit] trait
#[proc_macro_derive(OperateMultiQubit)]
pub fn derive_operate_multi_qubit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_n_qubit::dispatch_struct_enum_multi_qubit(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperatePragma] trait
#[proc_macro_derive(OperatePragma)]
pub fn derive_operate_pragmas(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_operate_pragma(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperatePragmaNoise] trait
#[proc_macro_derive(OperatePragmaNoise)]
pub fn derive_operate_noise_pragmas(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_operate_noise_pragma(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperatePragmaNoiseProba] trait
#[proc_macro_derive(OperatePragmaNoiseProba)]
pub fn derive_operate_noise_proba_pragmas(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_operate_noise_proba_pragma(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateGate] trait
#[proc_macro_derive(OperateGate)]
pub fn derive_operate_gate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_operate_gate(parsed_input).into()
}

/// Derive macro for the [roqoqo::Rotate] trait
#[proc_macro_derive(Rotate)]
pub fn derive_rotate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_rotate(parsed_input).into()
}

/// Derive macro for the [roqoqo::Define] trait
#[proc_macro_derive(Define)]
pub fn derive_define(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_define(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateConstantGate] trait
#[proc_macro_derive(OperateConstantGate)]
pub fn derive_operate_constant_gate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_constant_gate(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateSingleQubitGate] trait
#[proc_macro_derive(OperateSingleQubitGate)]
pub fn derive_operate_single_qubit_gate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_single_qubit_gate(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateTwoQubitGate] trait
#[proc_macro_derive(OperateTwoQubitGate)]
pub fn derive_operate_two_qubit_gate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_two_qubit_gate(parsed_input).into()
}

/// Derive macro for the [roqoqo::OperateMultiQubitGate] trait
#[proc_macro_derive(OperateMultiQubitGate)]
pub fn derive_operate_multi_qubit_gate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    operate_unitary::dispatch_struct_enum_multi_qubit_gate(parsed_input).into()
}

fn extract_fields_with_types(ds: DataStruct) -> Vec<(Ident, Option<String>, Type)> {
    let fields = match ds {
        DataStruct {
            fields: Fields::Named(fields),
            ..
        } => fields,
        _ => panic!("Trait can only be derived on structs with named fields"),
    };
    fields.named.into_iter().map(|f| {
        let id = f
            .ident
            .expect("Operate can only be derived on structs with named fields");
        let ty = f.ty;
        let type_path =match &ty {
            Type::Path(TypePath{path:p,..}) => p,
            _ => panic!("Trait  only supports fields with normal types of form path (e.g. CalculatorFloat, qoqo_calculator::CalculatorFloat)")
        };
        let mut type_string = match type_path.get_ident(){
            Some(ident_path) => Some(ident_path.to_string()),
            _ => type_path
            .segments
            .last().map(|segment|{segment.ident.to_string()})
        };
        if let Some(ref x) = type_string{
             if x.as_str() == "Option"{
            let inner_type = match &type_path.segments.iter().next().unwrap().arguments{
                PathArguments::AngleBracketed(angle_argumnets) =>  match angle_argumnets.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(TypePath{path:innerty,..})) => match innerty.get_ident(){
                        Some(ident_path) => Some(ident_path.to_string()),
                        _ =>innerty
                        .segments
                        .last().map(|segment|{segment.ident.to_string()})
                    },
                    _ => panic!("Expected GenericArgument")
                },
            _ => panic!("Expected AngleBracketed")
        };
        if let Some(s) = inner_type { if s.as_str() == "Circuit"{ 
            type_string = Some("Option<Circuit>".to_string())
        }}}
    }
        (id, type_string, ty)
    }).collect()
}

fn extract_variants_with_types(de: DataEnum) -> Vec<(Ident, Option<String>, Type)> {
    let DataEnum { variants, .. } = de;
    variants.into_iter().map(|v| {
        let vident = v.ident.clone();
        let fields = match v.fields {
            Fields::Unnamed(fields) => fields,
            _ => panic!("Trait can only be derived for enums with newtype structs as variants"),
        };
        if fields.unnamed.iter().len() != 1 {
            panic!("Trait can only be derived for enums with newtype structs as variants")
        }
        let ty = fields
            .unnamed
            .first()
            .expect("Trait can only be derived for enums with newtype structs as variants")
            .ty
            .clone();
        let type_path =match &ty {
            Type::Path(TypePath{path:p,..}) => p,
            _ => panic!("Trait only supports newtype variants with normal types of form path (e.g. CalculatorFloat, qoqo_calculator::CalculatorFloat)")
        };
        let type_string = match type_path.get_ident(){
            Some(ident_path) => Some(ident_path.to_string()),
            _ => type_path
            .segments
            .last().map(|segment|{segment.ident.to_string()})
        };
        (vident, type_string ,ty)
    }).collect()
}
