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

use quote::quote;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{AttrStyle, File, Ident, ItemImpl, ItemStruct, Path, Token, Type, TypePath};
/// Visitor scanning rust source code for struct belonging to enums
struct Visitor {
    // Identifiers of structs belonging to Operation enum
    operations: Vec<Ident>,
    // Identifiers of structs belonging to SingleQubitOperation enum
    single_qubit_operations: Vec<Ident>,
    // Identifiers of structs belonging to TwoQubitOperation enum
    two_qubit_operations: Vec<Ident>,
    // Identifiers of structs belonging to MultiQubitOperation enum
    multi_qubit_operations: Vec<Ident>,
    // Identifiers of structs belonging to PragmaOperation enum
    pragma_operations: Vec<Ident>,
    // Identifiers of structs belonging to PragmaNoiseOperation enum
    pragma_noise_operations: Vec<Ident>,
    // Identifiers of structs belonging to PragmaNoiseProbaOperation enum
    pragma_noise_proba_operations: Vec<Ident>,
    // Identifiers of structs belonging to GateOperation enum
    gate_operations: Vec<Ident>,
    // Identifiers of structs belonging to Rotation enum
    rotations: Vec<Ident>,
    // Identifiers of structs belonging to Definition enum
    definitions: Vec<Ident>,
    // Identifiers of structs belonging to ConstantGateOperation enum
    constant_gate_operations: Vec<Ident>,
    // Identifiers of structs belonging to SingleQubitGateOperation enum
    single_qubit_gate_operations: Vec<Ident>,
    // Identifiers of structs belonging to TwoQubitGateOperation enum
    two_qubit_gate_operations: Vec<Ident>,
    // Identifiers of structs belonging to MultiQubitGateOperation enum
    multi_qubit_gate_operations: Vec<Ident>,
    // Operations that have only been introduced in roqoqoq 1.1.0
    // These operations will only be added at end of automatically created enums
    // to maintain compatibility with bincode encoding
    roqoqo_1_1_operations: Vec<Ident>,
    // Operations that have only been introduced in roqoqoq 1.2.0
    // These operations will only be added at end of automatically created enums
    // to maintain compatibility with bincode encoding
    roqoqo_1_2_operations: Vec<Ident>,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            single_qubit_operations: Vec::new(),
            two_qubit_operations: Vec::new(),
            multi_qubit_operations: Vec::new(),
            pragma_operations: Vec::new(),
            pragma_noise_operations: Vec::new(),
            pragma_noise_proba_operations: Vec::new(),
            gate_operations: Vec::new(),
            rotations: Vec::new(),
            definitions: Vec::new(),
            constant_gate_operations: Vec::new(),
            single_qubit_gate_operations: Vec::new(),
            two_qubit_gate_operations: Vec::new(),
            multi_qubit_gate_operations: Vec::new(),
            roqoqo_1_1_operations: Vec::new(),
            roqoqo_1_2_operations: Vec::new(),
        }
    }
}

/// Struct for parsed derive macro arguments. Used to identify structs belonging to enums
#[derive(Debug)]
struct DeriveMacroArguments(HashSet<String>);

impl DeriveMacroArguments {
    pub fn contains(&self, st: &str) -> bool {
        self.0.contains(st)
    }
}

impl Parse for DeriveMacroArguments {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse arguments as comma separated list allowing for normal identifiers (like Debug) and colon
        // separated paths (like roqoqo::derive::Operate)
        let arguments = Punctuated::<Path, Token![,]>::parse_terminated(input)?;
        Ok(Self(
            arguments
                .into_iter()
                .map(|p| match p.get_ident() {
                    Some(id) => id.to_string(),
                    _ => p
                        .segments
                        .last()
                        .expect("Last path segment can not be accessed")
                        .ident
                        .to_string(),
                })
                .collect(),
        ))
    }
}

impl<'ast> Visit<'ast> for Visitor {
    // Only visit struct declarations
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        // Check attributes
        for att in i.attrs.clone() {
            let path = att.path.get_ident().map(|id| id.to_string());
            // only consider the derive attribute, if no derive attribute is present don't add anything
            // to the internal storage of the visitor
            if matches!(att.style, AttrStyle::Outer) && path == Some("derive".to_string()) {
                //let tokens: TokenStream = att.tokens.into();
                let parsed_arguments: DeriveMacroArguments =
                    att.parse_args().expect("parsing failed 1");
                // If the parsed arguments contains Operate it derives Operate and the identifier of the enum
                // is added to the internal list of the visitor
                if parsed_arguments.contains("Operate") {
                    self.operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate")
                    && parsed_arguments.contains("OperateSingleQubit")
                {
                    self.single_qubit_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate")
                    && parsed_arguments.contains("OperateTwoQubit")
                {
                    self.two_qubit_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate")
                    && parsed_arguments.contains("OperateMultiQubit")
                {
                    self.multi_qubit_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate")
                    && parsed_arguments.contains("OperatePragma")
                {
                    self.pragma_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate")
                    && parsed_arguments.contains("OperatePragma")
                    && parsed_arguments.contains("OperatePragmaNoise")
                {
                    self.pragma_noise_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate")
                    && parsed_arguments.contains("OperatePragma")
                    && parsed_arguments.contains("OperatePragmaNoise")
                    && parsed_arguments.contains("OperatePragmaNoiseProba")
                {
                    self.pragma_noise_proba_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate") && parsed_arguments.contains("OperateGate")
                {
                    self.gate_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Rotate") {
                    self.rotations.push(i.ident.clone());
                }
                if parsed_arguments.contains("Define") {
                    self.definitions.push(i.ident.clone());
                }
                if parsed_arguments.contains("Operate")
                    && parsed_arguments.contains("OperateConstantGate")
                {
                    self.constant_gate_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("OperateSingleQubitGate") {
                    self.single_qubit_gate_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("OperateGate")
                    && parsed_arguments.contains("OperateTwoQubit")
                {
                    self.two_qubit_gate_operations.push(i.ident.clone());
                }
                if parsed_arguments.contains("OperateMultiQubitGate") {
                    self.multi_qubit_gate_operations.push(i.ident.clone());
                }
            }
        }

        visit::visit_item_struct(self, i);
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        // Check implementation
        if let Some((_, trait_path, _)) = i.trait_.clone() {
            let trait_name = match trait_path.get_ident() {
                Some(id) => id.to_string(),
                _ => trait_path
                    .segments
                    .last()
                    .expect("Last path segment can not be accessed")
                    .ident
                    .to_string(),
            };
            if let Type::Path(TypePath { path: p, .. }) = *i.self_ty.clone() {
                let id = match p.get_ident() {
                    Some(id) => id.clone(),
                    _ => p
                        .segments
                        .last()
                        .expect("Last path segment can not be accessed")
                        .ident
                        .clone(),
                };

                if trait_name.as_str() == "Operate" {
                    self.operations.push(id.clone());
                }
                if trait_name.as_str() == "ImplementedIn1point1" {
                    self.roqoqo_1_1_operations.push(id.clone());
                }
                if trait_name.as_str() == "ImplementedIn1point2" {
                    self.roqoqo_1_2_operations.push(id.clone());
                }
                if trait_name.as_str() == "OperateSingleQubitGate" {
                    self.single_qubit_gate_operations.push(id.clone());
                }
                if trait_name.as_str() == "OperateGate" {
                    self.gate_operations.push(id.clone());
                }
                if trait_name.as_str() == "OperateTwoQubitGate" {
                    self.two_qubit_gate_operations.push(id.clone());
                }
                if trait_name.as_str() == "OperatePragmaNoise" {
                    self.pragma_noise_operations.push(id.clone());
                }
                if trait_name.as_str() == "OperatePragmaNoiseProba" {
                    self.pragma_noise_proba_operations.push(id.clone());
                }
                if trait_name.as_str() == "OperateMultiQubitGate" {
                    self.multi_qubit_gate_operations.push(id);
                }
            }
        }
        visit::visit_item_impl(self, i);
    }
}

const SOURCE_FILES: &[&str] = &[
    "src/operations/single_qubit_gate_operations.rs",
    "src/operations/pragma_operations.rs",
    "src/operations/two_qubit_gate_operations.rs",
    "src/operations/multi_qubit_gate_operations.rs",
    "src/operations/measurement_operations.rs",
    "src/operations/define_operations.rs",
];

fn main() {
    // create a visitor that will go through source code and collect the identifiers of structs that belong ad variants
    // in the Operation enum, those that belong in the SingleQubitGateOperationEnum and so on
    let mut vis = Visitor::new();
    // iterate over all source files where Operations are supposed to be located
    for source_location in SOURCE_FILES {
        let source = fs::read_to_string(source_location).expect("Unable to open source file");
        let code = proc_macro2::TokenStream::from_str(&source).expect("Could not lex code");
        let syntax_tree: File = syn::parse2(code).unwrap();
        vis.visit_file(&syntax_tree);
    }

    // Construct TokenStreams for variants of operation enum
    let operations_quotes = vis
        .operations
        .clone()
        .into_iter()
        .filter(|v| {
            !vis.roqoqo_1_1_operations.contains(v) && !vis.roqoqo_1_2_operations.contains(v)
        })
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
            #[allow(clippy::upper_case_acronyms)]
            #[doc = #msg]
            #v(#v)}
        });
    let operations_quotes_1_1 = vis
        .operations
        .clone()
        .into_iter()
        .filter(|v| vis.roqoqo_1_1_operations.contains(v) && !vis.roqoqo_1_2_operations.contains(v))
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
            #[allow(clippy::upper_case_acronyms)]
            #[doc = #msg]
            #v(#v)}
        });
    let operations_quotes_1_2 = vis
        .operations
        .into_iter()
        .filter(|v| vis.roqoqo_1_2_operations.contains(v))
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
            #[allow(clippy::upper_case_acronyms)]
            #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of operation enum
    let single_qubit_operations_quotes = vis.single_qubit_operations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
        #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of operation enum
    let two_qubit_operations_quotes = vis.two_qubit_operations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
        #[allow(clippy::upper_case_acronyms)]
        #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of operation enum
    let multi_qubit_operations_quotes = vis.multi_qubit_operations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
        #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of pragma enum
    let pragma_operations_quotes = vis
        .pragma_operations
        .clone()
        .into_iter()
        .filter(|v| {
            !vis.roqoqo_1_1_operations.contains(v) && !vis.roqoqo_1_2_operations.contains(v)
        })
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
            #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of pragma enum
    let pragma_operations_quotes_1_1 = vis
        .pragma_operations
        .clone()
        .into_iter()
        .filter(|v| vis.roqoqo_1_1_operations.contains(v) && !vis.roqoqo_1_2_operations.contains(v))
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
            #[doc = #msg]
            #v(#v)}
        });
    let pragma_operations_quotes_1_2 = vis
        .pragma_operations
        .into_iter()
        .filter(|v| vis.roqoqo_1_2_operations.contains(v))
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
            #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of pragma enum
    let pragma_noise_operations_quotes = vis.pragma_noise_operations.clone().into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
        #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of pragma enum
    let pragma_noise_proba_operations_quotes =
        vis.pragma_noise_proba_operations.into_iter().map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
            #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of pragma enum
    let gate_operations_quotes = vis.gate_operations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
        #[allow(clippy::upper_case_acronyms)]
        #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of definition enum
    let rotations_quotes = vis.rotations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
            #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of definition enum
    let definitions_quotes = vis
        .definitions
        .clone()
        .into_iter()
        .filter(|v| {
            !vis.roqoqo_1_1_operations.contains(v) && !vis.roqoqo_1_2_operations.contains(v)
        })
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
                #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of definition enum
    let definitions_quotes_1_1 = vis
        .definitions
        .clone()
        .into_iter()
        .filter(|v| vis.roqoqo_1_1_operations.contains(v) && !vis.roqoqo_1_2_operations.contains(v))
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
                #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of definition enum
    let definitions_quotes_1_2 = vis
        .definitions
        .into_iter()
        .filter(|v| vis.roqoqo_1_2_operations.contains(v))
        .map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
                #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of operation enum
    let constant_gate_operations_quote = vis.constant_gate_operations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
        #[allow(clippy::upper_case_acronyms)]
        #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of operation enum
    let single_qubit_gate_operations_quote =
        vis.single_qubit_gate_operations.into_iter().map(|v| {
            let msg = format!("Variant for {}", v);
            quote! {
                #[doc = #msg]
            #v(#v)}
        });
    // Construct TokenStreams for variants of operation enum
    let two_qubit_gate_operations_quote = vis.two_qubit_gate_operations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
        #[allow(clippy::upper_case_acronyms)]
        #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStreams for variants of operation enum
    let multi_qubit_gate_operations_quote = vis.multi_qubit_gate_operations.into_iter().map(|v| {
        let msg = format!("Variant for {}", v);
        quote! {
            #[doc = #msg]
        #v(#v)}
    });
    // Construct TokenStream for auto-generated rust file containing the enums
    let final_quote = quote! {

        //use crate::operations::*;

        /// Enum of all Operations implementing [Operate]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, Substitute, SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum Operation {
            #(#operations_quotes),* ,
            #(#operations_quotes_1_1),* ,
            #(#operations_quotes_1_2),*
        }

        /// Enum of all Operations implementing [OperateSingleQubit]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateSingleQubit, SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum SingleQubitOperation {
            #(#single_qubit_operations_quotes),*
        }

        /// Enum of all Operations implementing [OperateTwoQubit]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateTwoQubit,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum TwoQubitOperation {
            #(#two_qubit_operations_quotes),*
        }

        /// Enum of all Operations implementing [OperateMultiQubit]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateMultiQubit,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum MultiQubitOperation {
            #(#multi_qubit_operations_quotes),*
        }

        /// Enum of all Operations implementing [OperatePragma]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperatePragma,  SupportedVersion)]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        pub enum PragmaOperation {
            #(#pragma_operations_quotes),* ,
            #(#pragma_operations_quotes_1_1),* ,
            #(#pragma_operations_quotes_1_2),*
        }

        /// Enum of all Operations implementing [OperatePragmaNoise]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperatePragma, OperatePragmaNoise,  SupportedVersion)]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        pub enum PragmaNoiseOperation {
            #(#pragma_noise_operations_quotes),*
        }

        /// Enum of all Operations implementing [OperatePragmaNoiseProba]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperatePragma, OperatePragmaNoise, OperatePragmaNoiseProba,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum PragmaNoiseProbaOperation {
            #(#pragma_noise_proba_operations_quotes),*
        }

        /// Enum of all Operations implementing [OperateGate]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateGate,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum GateOperation {
            #(#gate_operations_quotes),*
        }

        /// Enum of all Operations implementing [Rotate]
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateGate, Rotate,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum Rotation {
            #(#rotations_quotes),*
        }

        /// Enum of all Operations implementing [Define]
        #[derive(Debug, Clone, PartialEq,InvolveQubits, Operate, OperateTryFromEnum, Substitute, Define,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum Definition {
            #(#definitions_quotes),* ,
            #(#definitions_quotes_1_1),* ,
            #(#definitions_quotes_1_2),*
        }

        /// Enum of all Operations implementing [OperateConstantGate]
        #[derive(Debug, Clone, PartialEq, Eq,InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateGate, OperateConstantGate,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum ConstantGateOperation {
            #(#constant_gate_operations_quote),*
        }

        /// Enum of all Operations implementing [OperateSingleQubitGate]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateGate, OperateSingleQubit, OperateSingleQubitGate,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum SingleQubitGateOperation {
            #(#single_qubit_gate_operations_quote),*
        }

        /// Enum of all Operations implementing [OperateTwoQubitGate]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateGate, OperateTwoQubit, OperateTwoQubitGate,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum TwoQubitGateOperation {
            #(#two_qubit_gate_operations_quote),*
        }

        /// Enum of all Operations implementing [OperateMultiQubitGate]
        #[derive(Debug, Clone, PartialEq, InvolveQubits, Operate, OperateTryFromEnum, Substitute, OperateGate, OperateMultiQubit, OperateMultiQubitGate,  SupportedVersion)]
        #[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
        // #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        pub enum MultiQubitGateOperation {
            #(#multi_qubit_gate_operations_quote),*
        }

    };
    let final_str = format!("{}", final_quote);
    let out_dir = PathBuf::from(
        std::env::var("OUT_DIR").expect("Cannot find a valid output directory for code generation"),
    )
    .join("_auto_generated_operations.rs");
    // Write to file
    fs::write(&out_dir, final_str).expect("Could not write to file");
    // Try to format auto generated operations
    let _unused_output = Command::new("rustfmt").arg(&out_dir).output();
}
