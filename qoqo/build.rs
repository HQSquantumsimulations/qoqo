// Copyright © 2020-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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
use pyo3::types::{PyAnyMethods, PyDict, PyModule};
use pyo3::{PyResult, Python};
use quote::{format_ident, quote};
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
    "src/operations/multi_qubit_gate_operations.rs",
    "src/operations/measurement_operations.rs",
    "src/operations/define_operations.rs",
    "src/operations/bosonic_operations.rs",
    "src/operations/spin_boson_operations.rs",
    #[cfg(feature = "unstable_analog_operations")]
    "src/operations/analog_operations.rs",
];

fn str_to_type(res: &str) -> Option<String> {
    match res {
        s if s.contains("Pragma") => Some("Operation".to_owned()),
        "CalculatorFloat" => Some("Tuple[float, str]".to_owned()),
        "String" | "string" => Some("str".to_owned()),
        "" => None,
        "uint" => Some("int".to_owned()),
        "self" => Some("Circuit".to_owned()),
        _ => Some(
            res.replace("list", "List")
                .replace("dict", "Dict")
                .replace("tuple", "Tuple")
                .replace("set", "Set")
                .replace("circuit", "Circuit")
                .replace("optional", "Optional")
                .replace("operation", "Operation")
                .to_owned(),
        ),
    }
}

fn extract_type(string: &str) -> Option<String> {
    let pattern = r"\(([a-zA-Z_\[\] ,|]+?)\)";
    let re = Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(string) {
        if let Some(res) = captures.get(1).map(|s| s.as_str()) {
            return str_to_type(res);
        }
    }
    None
}
fn collect_args_from_doc(doc: &str) -> Vec<String> {
    let args_vec: Vec<_> = doc
        .split("\n")
        .skip_while(|&line| line != "Args:")
        .skip(1)
        .skip_while(|line| line.len() == 0)
        .take_while(|line| line.len() != 0)
        .collect();
    args_vec
        .iter()
        .filter_map(|&line| {
            (line.contains(':') && line.trim().starts_with(char::is_alphabetic)).then(|| {
                format!(
                    "{}{}",
                    line.trim().split_once([' ', ':']).unwrap_or(("", "")).0,
                    extract_type(line)
                        .map(|arg_type| format!(": {}", arg_type))
                        .unwrap_or_default()
                )
            })
        })
        .collect()
}

fn collect_return_from_doc(doc: &str) -> String {
    let args_vec: Vec<_> = doc
        .split("\n")
        .skip_while(|&line| line != "Returns:")
        .skip(1)
        .take(1)
        .filter(|&line| line.contains(':') && line.trim().starts_with(char::is_alphabetic))
        .collect();
    if args_vec.len() == 0 {
        "".to_owned()
    } else if let Some(ret) =
        str_to_type(args_vec[0].trim().split_once([':']).unwrap_or(("", "")).0)
    {
        format!(" -> {}", ret)
    } else {
        "".to_owned()
    }
}

fn create_doc(module: &str) -> PyResult<String> {
    let mut module_doc = "# This is an auto generated file containing only the documentation.\n# You can find the full implementation on this page:\n# https://github.com/HQSquantumsimulations/qoqo\n\n".to_owned();
    if module == "qoqo" {
        module_doc.push_str("from typing import Optional, List, Tuple, Dict, Set\n\n");
    } else {
        module_doc.push_str("from .qoqo import Circuit, Operation\nfrom typing import Tuple, List, Optional, Dict\n\n");
    };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<String> {
        let python_module = PyModule::import_bound(py, module)?;
        let dict = python_module.as_gil_ref().getattr("__dict__")?;
        let r_dict = dict.downcast::<PyDict>()?;
        for (fn_name, func) in r_dict.iter() {
            let name = fn_name.str()?.extract::<String>()?;
            if name.starts_with("__")
                || (module == "qoqo"
                    && ![
                        "qoqo",
                        "Circuit",
                        "QuantumProgram",
                        "CircuitDag",
                        "operations",
                    ]
                    .contains(&name.as_str()))
            {
                continue;
            }
            let doc = func.getattr("__doc__")?.extract::<String>()?;
            if name == "qoqo" {
                module_doc.push_str(&format!(
                    "def {name}({}):\n    \"\"\"\n{doc}\n\"\"\"\n\n",
                    collect_args_from_doc(doc.as_str()).join(", "),
                ));
            } else if name == "operations" {
                module_doc.push_str(&format!(
                    "class Operation:\n    \"\"\"\n{doc}\n\"\"\"\n\n    def __init__(self):\n       return\n\n",
                ));
            } else {
                let args = collect_args_from_doc(doc.as_str()).join(", ");
                module_doc.push_str(&format!(
                    "class {name}{}:\n    \"\"\"\n{doc}\n\"\"\"\n\n    def __init__(self{}):\n       return\n\n",
                    module.eq("qoqo.operations").then_some("(Operation)").unwrap_or_default(),
                    args.is_empty().then_some("").unwrap_or(format!(", {}", args).as_str()),
                ));
                let class_dict = func.getattr("__dict__")?;
                let items = class_dict.call_method0("items")?;
                let dict_obj = py
                    .import_bound("builtins")?
                    .call_method1("dict", (items,))?;
                let class_r_dict = dict_obj.as_gil_ref().downcast::<PyDict>()?;
                for (class_fn_name, meth) in class_r_dict.iter() {
                    let meth_name = class_fn_name.str()?.extract::<String>()?;
                    let class_doc = match meth_name.as_str() {
                        "__add__" => r#"Implement the `+` (__add__) magic method to add two Circuits.

Args:
    rhs (Operation | Circuit): The second Circuit object in this operation.

Returns:
    Circuit: self + rhs the two Circuits added together.

    Raises:
    TypeError: Left hand side can not be converted to Circuit.
    TypeError: Right hand side cannot be converted to Operation or Circuit."#.to_owned(),
                        "__iadd__" => r#"Implement the `+=` (__iadd__) magic method to add a Operation to a Circuit.

Args:
    other (Operation | Circuit): The Operation object to be added to self.

Returns:
    Circuit: self + other the two Circuits added together as the first one.

Raises:
    TypeError: Right hand side cannot be converted to Operation or Circuit."#.to_owned(),
                        "__new__" => "".to_owned(),
                        _ => meth
                            .getattr("__doc__")
                            ?
                            .extract::<String>()
                            .unwrap_or_default(),
                    };
                    if class_doc.eq("") {
                        continue;
                    }
                    let meth_args = collect_args_from_doc(class_doc.as_str()).join(", ");
                    module_doc.push_str(&format!(
                        "    @classmethod\n    def {meth_name}(self{}){}: # type: ignore\n        \"\"\"\n{class_doc}\n\"\"\"\n\n",
                        meth_args.is_empty().then_some("").unwrap_or(format!(", {}", meth_args).as_str()),
                        collect_return_from_doc(class_doc.as_str())
                    ));
                }
            }
        }
        Ok(module_doc)
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
                                    let tmp: Option<&PyAny> = #pyobject_name.extract().map_err(|_|
                                        QoqoError::ConversionError)?;
                                    let #ident = match tmp{
                                        Some(cw) => Some(convert_into_circuit(&cw.as_borrowed())
                                        .map_err(|_| QoqoError::ConversionError)?),
                                        _ => None
                                    };
                                }},
                                "SpinHamiltonian" => {quote!{
                                    let #pyobject_name = &op
                                    .call_method0(#ident_string)
                                    .map_err(|_| QoqoError::ConversionError)?;
                                    let temp_op: struqture::spins::SpinHamiltonianSystem = struqture_py::spins::SpinHamiltonianSystemWrapper::from_pyany((#pyobject_name.as_any().clone()).into()).map_err(|_| QoqoError::ConversionError)?;
                                    let #ident = temp_op.hamiltonian().clone();
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
        use ndarray::Array1;
        use num_complex::Complex64;
        use numpy::{PyArray2, PyReadonlyArray1};

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

    for &module in [
        "qoqo",
        "operations",
        "measurements",
        "noise_models",
        "devices",
    ]
    .iter()
    {
        let qoqo_doc = create_doc(
            module
                .eq("qoqo")
                .then_some(module)
                .unwrap_or(&format!("qoqo.{module}")),
        )
        .expect("Could not generate documentation.");
        let out_dir = PathBuf::from(format!("qoqo/{}.pyi", module));
        fs::write(&out_dir, qoqo_doc).expect("Could not write to file");
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
        let type_path =match &ty {
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
