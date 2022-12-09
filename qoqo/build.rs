// Copyright © 2020-2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use quote::{format_ident, quote};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{
    AttrStyle, Fields, File, GenericArgument, Ident, ItemStruct, Macro, Path, PathArguments, Token,
    Type, TypePath,
};

type StructFieldInfo = Vec<(Ident, Option<String>, Type)>;
type WrappedStructInfo = Vec<(Ident, Ident, StructFieldInfo)>;

/// Visitor scanning rust source code for struct belonging to enums
struct Visitor {
    /// Information for structs with the wrap attribute macro
    info_wrap: WrappedStructInfo,
    /// Code for conversion pyany -> operation to be injected into convert_pyany_to_operation
    pyany_to_operation: Vec<TokenStream>,
    /// Code for conversion operation -> pyobject to be injected into convert_operation_to_pyobject
    operation_to_pyobject: Vec<TokenStream>,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            info_wrap: Vec::new(),
            pyany_to_operation: Vec::new(),
            operation_to_pyobject: Vec::new(),
        }
    }
}

/// Struct for parsed derive macro arguments. Used to identify structs belonging to enums
#[derive(Debug)]
struct DeriveMacroArguments(HashSet<String>);

impl DeriveMacroArguments {
    pub fn _contains(&self, st: &str) -> bool {
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
    fn visit_item_struct(&mut self, itemstruct: &'ast ItemStruct) {
        // Check attributes
        for att in itemstruct.attrs.clone() {
            let path = att.path.get_ident().map(|id| id.to_string());
            // only consider the wrap attribute, if no derive attribute is present don't add anything
            // to the internal storage of the visitor
            if att.style == AttrStyle::Outer && path == Some("wrap".to_string()) {
                let wrapper_ident = format_ident!("{}Wrapper", itemstruct.ident);
                let field_information = extract_fields_with_types(itemstruct.fields.clone());
                self.info_wrap
                    .push((itemstruct.ident.clone(), wrapper_ident, field_information));
            }
        }

        visit::visit_item_struct(self, itemstruct);
    }
    fn visit_macro(&mut self, i: &'ast Macro) {
        let id = match i.path.clone().get_ident() {
            Some(id) => Some(id.clone()),
            _ => i.path.segments.last().map(|segment| segment.ident.clone()),
        };
        if let Some(ident) = id {
            if ident.to_string().as_str() == "insert_pyany_to_operation" {
                self.pyany_to_operation.push(i.tokens.clone())
            }
            if ident.to_string().as_str() == "insert_operation_to_pyobject" {
                self.operation_to_pyobject.push(i.tokens.clone())
            }
        }
        visit::visit_macro(self, i);
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
    pyo3_build_config::add_extension_module_link_args();
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
    let pyany_to_operation_quotes =
        vis.info_wrap
            .clone()
            .into_iter()
            .map(|(ident, _wrapper_ident, field_information)| {
                let ident_string = ident.to_string();
                let arguments = field_information.iter().map(|(id, _, _)| {
                    quote! {#id}
                });
                let field_quotes = field_information.iter().map(|(ident, type_string, ty)| {
                    let pyobject_name = format_ident!("{}_pyobject", ident);
                    let ident_string = ident.to_string();
                    match type_string {
                        Some(type_str) =>
                            match  type_str.as_str(){
                                "CalculatorFloat" => {quote!{
                                    let #pyobject_name = op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let #ident = convert_into_calculator_float(#pyobject_name).map_err(|_|
                                        QoqoError::ConversionError)?;
                                }},
                                "Circuit" => {quote!{
                                    let #pyobject_name = op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let #ident = convert_into_circuit(#pyobject_name).map_err(|_|
                                        QoqoError::ConversionError)?;
                                }},
                                "Option<Circuit>" => {quote!{
                                    let #pyobject_name = op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let tmp: Option<&PyAny> = #pyobject_name.extract().map_err(|_|
                                        QoqoError::ConversionError)?;
                                    let #ident = match tmp{
                                        Some(cw) => Some(convert_into_circuit(cw)
                                        .map_err(|_| QoqoError::ConversionError)?),
                                        _ => None
                                    };
                                }},
                                _ => {
                                    quote!{
                                    let #pyobject_name = op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let #ident: #ty = #pyobject_name.extract()
                                    .map_err(|_| QoqoError::ConversionError)?;
                                }}
                            },
                        None => {
                            quote!{
                                let #pyobject_name = op
                                .call_method0(#ident_string)
                                .map_err(|_| QoqoError::ConversionError)?;
                                let #ident: #ty = #pyobject_name.extract()
                                    .map_err(|_| QoqoError::ConversionError)?;
                            }
                         }
                    }
                }
                );
                quote! {#ident_string => {
                    #(#field_quotes)*
                    Ok(#ident::new(#(#arguments),*).into())
                    }
                }
            });

    let operation_to_pyobject_quotes =
        vis.info_wrap
            .into_iter()
            .map(|(ident, wrapper_ident, _field_information)| {
                quote! {
                    Operation::#ident(internal) => {
                        let pyref: Py<#wrapper_ident> =
                            Py::new(py, #wrapper_ident { internal }).unwrap();
                        let pyobject: PyObject = pyref.to_object(py);
                        Ok(pyobject)
                    }
                }
            });

    let operation_to_pyobject_injected_quotes: Vec<TokenStream> = vis.operation_to_pyobject;
    let pyany_to_operation_injected_quotes: Vec<TokenStream> = vis.pyany_to_operation;

    // Construct TokenStream for auto-generated rust file containing the enums
    let final_quote = quote! {
        use crate::QoqoError;
        use crate::convert_into_circuit;
        use qoqo_calculator::CalculatorFloat;
        use qoqo_calculator_pyo3::convert_into_calculator_float;
        use pyo3::conversion::ToPyObject;
        use roqoqo::operations::*;
        use std::collections::HashMap;
        use ndarray::Array1;
        use num_complex::Complex64;
        use numpy::{PyArray2, PyReadonlyArray1};

        /// Tries to convert a [roqoqo::operations::Operation] to a PyObject
        pub fn convert_operation_to_pyobject(operation: Operation) -> PyResult<PyObject> {
            Python::with_gil(|py| -> PyResult<PyObject> {
            match operation {
                #(#operation_to_pyobject_quotes),*
                #(#operation_to_pyobject_injected_quotes),*
            }
        })
        }

        /// Tries to convert any python object to a [roqoqo::operations::Operation]
        pub fn convert_pyany_to_operation(op: &PyAny) -> Result<Operation, QoqoError> {
            let hqslang_pyobject = op
                .call_method0("hqslang")
                .map_err(|_| QoqoError::ConversionError)?;
            let hqslang: String = String::extract(hqslang_pyobject)
                .map_err(|_| QoqoError::ConversionError)?;
            match hqslang.as_str() {
                #(#pyany_to_operation_quotes),*
                #(#pyany_to_operation_injected_quotes),*
                _ => Err(QoqoError::ConversionError),
            }
        }
    };
    let final_str = format!("{}", final_quote);
    // Don't write to file when running on docs.rs
    let out_dir = PathBuf::from(
        std::env::var("OUT_DIR").expect("Cannot find a valid output directory for code generation"),
    )
    .join("_auto_generated_operation_conversion.rs");
    // Write to file
    fs::write(&out_dir, final_str).expect("Could not write to file");
    // Try to format auto generated operations
    let _unused_output = Command::new("rustfmt").arg(&out_dir).output();
}

fn extract_fields_with_types(input_fields: Fields) -> Vec<(Ident, Option<String>, Type)> {
    let fields = match input_fields {
        Fields::Named(fields) => fields,
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
