// Copyright © 2020-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
#[cfg(feature = "doc_generator")]
use pyo3::{
    types::{PyAnyMethods, PyDict, PyModule},
    {PyResult, Python},
};
use quote::{format_ident, quote};
#[cfg(feature = "doc_generator")]
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{
    AttrStyle, Fields, File, GenericArgument, Ident, ItemStruct, LitStr, Macro, Path,
    PathArguments, Token, Type, TypePath,
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

#[derive(Debug, Eq, PartialEq)]
struct CfgFeatureMacroArgument(String);

impl Parse for CfgFeatureMacroArgument {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let feature_name: LitStr = input.parse()?;
        Ok(Self(feature_name.value()))
    }
}

/// Struct for parsed derive macro arguments. Used to identify structs belonging to enums
#[allow(dead_code)]
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
            let path = att.path().get_ident().map(|id| id.to_string());
            // TEMP: REMOVE WHEN STABILISED
            if matches!(att.style, AttrStyle::Outer)
                && path == Some("cfg".to_string())
                && !cfg!(feature = "unstable_operation_definition")
            {
                let cfg_feature_name: CfgFeatureMacroArgument =
                    att.parse_args().expect("parsing failed 1");
                if cfg_feature_name.0.contains("unstable_operation_definition") {
                    return;
                }
            }

            // TEMP: REMOVE WHEN STABILISED
            if matches!(att.style, AttrStyle::Outer)
                && path == Some("cfg".to_string())
                && !cfg!(feature = "unstable_simulation_repetitions")
            {
                let cfg_feature_name: CfgFeatureMacroArgument =
                    att.parse_args().expect("parsing failed 1");
                if cfg_feature_name
                    .0
                    .contains("unstable_simulation_repetitions")
                {
                    return;
                }
            }

            // only consider the wrap attribute, if no derive attribute is present don't add anything
            // to the internal storage of the visitor
            if matches!(att.style, AttrStyle::Outer) && path == Some("wrap".to_string()) {
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
        // TEMP: REMOVE WHEN STABILISED
        if i.tokens.clone().into_iter().any(|tok| {
            tok.to_string().contains("CallDefinedGate")
                || tok.to_string().contains("DefinitionGate")
        }) && !cfg!(feature = "unstable_operation_definition")
        {
            return;
        }

        // TEMP: REMOVE WHEN STABILISED
        if i.tokens
            .clone()
            .into_iter()
            .any(|tok| tok.to_string().contains("PragmaSimulationRepetitions"))
            && !cfg!(feature = "unstable_simulation_repetitions")
        {
            return;
        }

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
    "src/operations/three_qubit_gate_operations.rs",
    "src/operations/four_qubit_gate_operations.rs",
    "src/operations/multi_qubit_gate_operations.rs",
    "src/operations/measurement_operations.rs",
    "src/operations/define_operations.rs",
    "src/operations/bosonic_operations.rs",
    "src/operations/spin_boson_operations.rs",
    #[cfg(feature = "unstable_analog_operations")]
    "src/operations/analog_operations.rs",
];

#[cfg(feature = "doc_generator")]
fn str_to_type(res: &str, class_name: &str) -> Option<String> {
    match res {
        s if s.contains("Pragma") => Some("Operation".to_owned()),
        "CalculatorFloat" => Some("Union[float, str]".to_owned()),
        "String" | "string" => Some("str".to_owned()),
        "" => None,
        "uint" => Some("int".to_owned()),
        "self" | "Self" => Some(class_name.to_owned()),
        "ByteArray" => Some("bytearray".to_owned()),
        _ => Some(
            res.replace("Option[", "Optional[")
                .replace("operation", "Operation")
                .replace("np.", "numpy.")
                .replace("struqture_py.spins.", "")
                .to_owned(),
        ),
    }
}

#[cfg(feature = "doc_generator")]
fn extract_type(string: &str, class_name: &str) -> Option<String> {
    let pattern = r"\(([a-zA-Z_\[\]\. ,|]+?)\)";
    let re = Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(string) {
        if let Some(res) = captures.get(1).map(|s| s.as_str()) {
            return str_to_type(res, class_name);
        }
    }
    None
}

#[cfg(feature = "doc_generator")]
fn collect_args_from_doc(doc: &str, class_name: &str) -> Vec<String> {
    let args_vec: Vec<_> = doc
        .split('\n')
        .skip_while(|&line| line != "Args:")
        .skip(1)
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .collect();
    args_vec
        .iter()
        .filter(|&line| line.contains(':') && line.trim().starts_with(char::is_alphabetic))
        .map(|&line| {
            let arg_type = extract_type(line, class_name);
            format!(
                "{}{}",
                line.trim().split_once([' ', ':']).unwrap_or(("", "")).0,
                arg_type
                    .map(|arg_type| format!(": {}", arg_type))
                    .unwrap_or_default()
            )
        })
        .collect()
}

#[cfg(feature = "doc_generator")]
fn collect_return_from_doc(doc: &str, class_name: &str) -> String {
    let args_vec: Vec<_> = doc
        .split('\n')
        .skip_while(|&line| line != "Returns:")
        .skip(1)
        .take(1)
        .filter(|&line| line.contains(':') && line.trim().starts_with(char::is_alphabetic))
        .collect();
    if args_vec.is_empty() {
        "".to_owned()
    } else if let Some(ret) = str_to_type(
        args_vec[0].trim().split_once([':']).unwrap_or(("", "")).0,
        class_name,
    ) {
        format!(" -> {}", ret)
    } else {
        "".to_owned()
    }
}

#[cfg(feature = "doc_generator")]
const TYPING_POTENTIAL_IMPORTS: &[&str] = &[
    "Optional", "List", "Tuple", "Dict", "Set", "Union", "Sequence",
];
#[cfg(feature = "doc_generator")]
const STRUQTURE_POTENTIAL_IMPORTS: &[&str] = &[
    "PauliProduct",
    "DecoherenceProduct",
    "SpinSystem",
    "SpinHamiltonianSystem",
    "SpinLindbladNoiseSystem",
    "SpinLindbladOpenSystem",
    "PlusMinusProduct",
    "PlusMinusOperator",
    "PlusMinusLindbladNoiseOperator",
];
#[cfg(feature = "doc_generator")]
const QOQO_POTENTIAL_IMPORTS: &[&str] = &["Circuit", "Operation"];

#[cfg(feature = "doc_generator")]
fn create_doc(module: &str) -> PyResult<String> {
    let mut main_doc = "".to_owned();
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<String> {
        let python_module = PyModule::import(py, module)?;
        let dict = python_module.as_ref().getattr("__dict__")?;
        let module_doc = python_module
            .as_ref()
            .getattr("__doc__")?
            .extract::<String>()?;
        let r_dict = dict.downcast::<PyDict>()?;
        for (fn_name, func) in pyo3::types::PyDictMethods::iter(r_dict) {
            let name = fn_name.str()?.extract::<String>()?;
            if name.starts_with("__")
                || (module == "qoqo"
                    && !["Circuit", "QuantumProgram", "CircuitDag", "operations"]
                        .contains(&name.as_str()))
            {
                continue;
            }
            let doc = func.getattr("__doc__")?.extract::<String>()?;
            if name == "operations" {
                main_doc.push_str(&format!(
                    "class Operation:\n    \"\"\"\n{doc}\n\"\"\"\n\n    def __init__(self):\n       return\n\nclass Backend:\n    \"\"\"\nCan be any backend from a qoqo interface such as qoqo-qiskit, qoqo-quest or qoqo-qasm.\n\"\"\"\n\n",
                ));
            } else {
                let args = collect_args_from_doc(doc.as_str(), name.as_str()).join(", ");
                main_doc.push_str(&format!(
                    "class {name}{}:\n    \"\"\"\n{doc}\n\"\"\"\n\n    def __init__(self{}):\n       return\n\n",
                    module.eq("qoqo.operations").then_some("(Operation)").unwrap_or_default(),
                    if args.is_empty() { "".to_owned() } else { format!(", {}", args) },
                ));
                let class_dict = func.getattr("__dict__")?;
                let items = class_dict.call_method0("items")?;
                let dict_obj = py.import("builtins")?.call_method1("dict", (items,))?;
                let class_r_dict = dict_obj.as_ref().downcast::<PyDict>()?;
                for (class_fn_name, meth) in pyo3::types::PyDictMethods::iter(class_r_dict) {
                    let meth_name = class_fn_name.str()?.extract::<String>()?;
                    let meth_doc = match meth_name.as_str() {
                        "__add__" if name.eq(&"Circuit") => r#"Implement the `+` (__add__) magic method to add two Circuits.

Args:
    rhs (Operation | Circuit): The second Circuit object in this operation.

Returns:
    Circuit: self + rhs the two Circuits added together.

    Raises:
    TypeError: Left hand side can not be converted to Circuit.
    TypeError: Right hand side cannot be converted to Operation or Circuit."#.to_owned(),
                        "__iadd__" if name.eq(&"Circuit") => r#"Implement the `+=` (__iadd__) magic method to add a Operation to a Circuit.

Args:
    other (Operation | Circuit): The Operation object to be added to self.

Returns:
    Circuit: self + other the two Circuits added together as the first one.

Raises:
    TypeError: Right hand side cannot be converted to Operation or Circuit."#.to_owned(),
                        method if method.starts_with("__") => "".to_owned(),
                        _ => {
                            let tmp_doc = meth
                            .getattr("__doc__")
                            ?
                            .extract::<String>()
                            .unwrap_or_default();
                        if tmp_doc.starts_with("staticmethod(function) -> method") {
                            meth
                            .getattr("__func__")
                            ?.getattr("__doc__")?.extract::<String>().unwrap_or_default()
                        } else {
                            tmp_doc
                        }
                        }
                    };
                    if meth_doc.is_empty() {
                        continue;
                    }
                    let meth_args =
                        collect_args_from_doc(meth_doc.as_str(), name.as_str()).join(", ");
                    main_doc.push_str(&format!(
                        "    def {meth_name}(self{}){}:\n        \"\"\"\n{meth_doc}\n\"\"\"\n\n",
                        if meth_args.is_empty() {
                            "".to_owned()
                        } else {
                            format!(", {}", meth_args)
                        },
                        collect_return_from_doc(meth_doc.as_str(), name.as_str(),)
                    ));
                }
            }
        }
        let typing_imports: Vec<&str> = TYPING_POTENTIAL_IMPORTS
            .iter()
            .filter(|&type_str| main_doc.contains(&format!("{type_str}[")))
            .copied()
            .collect();
        let struqture_imports: Vec<&str> = STRUQTURE_POTENTIAL_IMPORTS
            .iter()
            .filter(|&type_str| {
                main_doc.contains(&format!("{type_str}:"))
                    || main_doc.contains(&format!(":{type_str}"))
                    || main_doc.contains(&format!("[{type_str}]"))
                    || main_doc.contains(&format!("({type_str})"))
                    || main_doc.contains(&format!("-> {type_str}"))
            })
            .copied()
            .collect();
        let qoqo_imports: Vec<&str> = QOQO_POTENTIAL_IMPORTS
            .iter()
            .filter(|&type_str| main_doc.contains(type_str))
            .copied()
            .collect();
        Ok(
            format!("# This is an auto generated file containing only the documentation.\n# You can find the full implementation on this page:\n# https://github.com/HQSquantumsimulations/qoqo\n\n\"\"\"\n{module_doc}\n\"\"\"\n\n{}{}{}{}\n{}",
        if main_doc.lines().any(|line| line.contains("numpy") && !line.contains("import")) { "import numpy\n" } else { "" },
        if typing_imports.is_empty() { "".to_owned() } else { format!("from typing import {}\n", typing_imports.join(", ")) },
        if struqture_imports.is_empty() { "".to_owned() } else { format!("from struqture_py.spins import {} \n", struqture_imports.join(", ")) },
        if module.eq("qoqo") || qoqo_imports.is_empty() { "".to_owned() } else {format!("from .qoqo import {}\n", qoqo_imports.join(", "))},
        main_doc
    ))
    })
}

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
                                    let #pyobject_name = &op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let #ident = convert_into_calculator_float(#pyobject_name).map_err(|_|
                                        QoqoError::ConversionError)?;
                                }},
                                "Circuit" => {quote!{
                                    let #pyobject_name = &op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let #ident = convert_into_circuit(#pyobject_name).map_err(|_|
                                        QoqoError::ConversionError)?;
                                }},
                                "Option<Circuit>" => {quote!{
                                    let #pyobject_name = &op
                                        .call_method0(#ident_string)
                                        .map_err(|_| QoqoError::ConversionError)?;
                                    let tmp: Option<&Bound<PyAny>> = #pyobject_name.into();
                                    let #ident = match tmp {
                                        Some(cw) => {
                                            if cw.is_none() {
                                                None
                                            } else {
                                                Some(convert_into_circuit(cw).map_err(|_| {
                                                    QoqoError::ConversionError
                                                })?)
                                            }
                                        },
                                        _ => None
                                    };
                                }},
                                "PauliHamiltonian" => {quote!{
                                    let #pyobject_name = &op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let #ident: struqture::spins::PauliHamiltonian = struqture_py::spins::PauliHamiltonianWrapper::from_pyany(#pyobject_name).map_err(|_| QoqoError::ConversionError)?;
                                }},
                                _ => {
                                    quote!{
                                    let #pyobject_name = &op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let #ident: #ty = #pyobject_name.extract()
                                    .map_err(|_| QoqoError::ConversionError)?;
                                }}
                            },
                        None => {
                            quote!{
                                let #pyobject_name = &op
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
        use ndarray::{Array1, Array2};
        use num_complex::Complex64;
        use numpy::{PyReadonlyArray1, PyReadonlyArray2};

        /// Tries to convert a [roqoqo::operations::Operation] to a PyObject
        pub fn convert_operation_to_pyobject(operation: Operation) -> PyResult<PyObject> {
            Python::with_gil(|py| -> PyResult<PyObject> {
            match operation {
                #(#operation_to_pyobject_quotes),*
                #(#operation_to_pyobject_injected_quotes),*
                _ => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Unknown operation: {}", operation.hqslang())))
            }
        })
        }

        /// Tries to convert any python object to a [roqoqo::operations::Operation]
        pub fn convert_pyany_to_operation(op: &Bound<PyAny>) -> Result<Operation, QoqoError> {
            let hqslang_pyobject = &op
                .call_method0("hqslang")
                .map_err(|_| QoqoError::ConversionError)?;
            let hqslang: String = String::extract_bound(hqslang_pyobject)
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

    #[cfg(feature = "doc_generator")]
    {
        for &module in [
            "qoqo",
            "operations",
            "measurements",
            "noise_models",
            "devices",
        ]
        .iter()
        {
            let qoqo_doc = if module.eq("qoqo") {
                create_doc(module)
            } else {
                create_doc(&format!("qoqo.{module}"))
            }
            .expect("Could not generate documentation.");
            let out_dir = PathBuf::from(format!("python/qoqo/{}.pyi", module));
            fs::write(&out_dir, qoqo_doc).expect("Could not write to file");
        }
    }
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
        let type_path = match &ty {
            Type::Path(TypePath{path:p,..}) => p,
            _ => panic!("Trait only supports fields with normal types of form path (e.g. CalculatorFloat, qoqo_calculator::CalculatorFloat)")
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
