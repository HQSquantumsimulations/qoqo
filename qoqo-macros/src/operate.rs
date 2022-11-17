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

use crate::{extract_fields_with_types, RESERVED_FIELDS};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{Data, DataStruct, DeriveInput, Ident};

/// Dispatch to derive Operate for enums and structs

pub fn dispatch_struct(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    match input.data {
        Data::Struct(ds) => operate_struct(ds, ident),
        _ => panic!("InvolveQubits can only be derived on structs"),
    }
}

/// Generate TokenStream of implementation of Operate for structs
fn operate_struct(ds: DataStruct, ident: Ident) -> TokenStream {
    let reserved_fields: HashSet<&str> = RESERVED_FIELDS.iter().cloned().collect();
    let fields_with_type = extract_fields_with_types(ds).into_iter();
    let input_arguments = fields_with_type
        .clone()
        .map(|(id, type_string, ty)| match type_string {
            Some(s) => match s.as_str() {
                "CalculatorFloat" => quote! {#id: &pyo3::PyAny},
                "Circuit" => quote! {#id: &pyo3::PyAny},
                "Option<Circuit>" => quote! {#id: &pyo3::PyAny},
                _ => quote! {#id: #ty},
            },
            _ => quote! {#id: #ty},
        });
    let arguments = fields_with_type
        .clone()
        .map(|(id, type_string, _)| match type_string {
            Some(s) => match s.as_str() {
                "CalculatorFloat" => {
                    let id_extracted = format_ident!("{}_extracted", id);
                    quote! {
                    #id_extracted}
                }
                "Circuit" => {
                    let id_extracted = format_ident!("{}_extracted", id);
                    quote! {
                    #id_extracted}
                }
                "Option<Circuit>" => {
                    let id_extracted = format_ident!("{}_extracted", id);
                    quote! {
                    #id_extracted}
                }
                _ => {
                    quote! {#id}
                }
            },
            _ => {
                quote! {#id}
            }
        });
    let conversion_quotes = fields_with_type
        .clone()
        .map(|(id, type_string, ty)| match type_string{
            Some(s) => match s.as_str() {
            "CalculatorFloat" => {
                let id_extracted = format_ident!("{}_extracted", id);
                quote! {
                    let #id_extracted: #ty = convert_into_calculator_float(#id).map_err(|x| {
                        pyo3::exceptions::PyTypeError::new_err(format!("Argument cannot be converted to CalculatorFloat {:?}",x))
                    })?;
                }
            },
            "Circuit" => {
                let id_extracted = format_ident!("{}_extracted", id);
                quote! {
                    let #id_extracted: #ty = convert_into_circuit(#id).map_err(|x| {
                        pyo3::exceptions::PyTypeError::new_err(format!("Argument cannot be converted to Circuit {:?}",x))
                    })?;
                }
            },
            "Option<Circuit>" => {
                let id_extracted = format_ident!("{}_extracted", id);
                quote! {
                    let tmp: Option<&PyAny> = #id.extract().map_err(|x| {
                        pyo3::exceptions::PyTypeError::new_err(format!("Argument cannot be converted to Circuit {:?}",x))
                    })?;
                    let #id_extracted: Option<Circuit> = match tmp{
                        Some(cw) => Some(convert_into_circuit(cw).map_err(|x| {
                            pyo3::exceptions::PyTypeError::new_err(format!("Argument cannot be converted to Circuit {:?}",x))
                    })?),
                        _ => None };
            }},
            _ => {
                quote! {}
            }
        },
        _ => {
            quote! {}
        }
        });
    let getter_fields = fields_with_type
        .filter(|(id, _, _)| !reserved_fields.contains(id.to_string().as_str()))
        .map(|(id, type_string, ty)| match type_string {
            Some(s) => match s.as_str() {
                "CalculatorFloat" => {
                    let msg = format!("Returns value of attribute {}", id);
                    quote! {
                        #[doc = #msg]
                        pub fn #id(&self) -> CalculatorFloatWrapper{
                            CalculatorFloatWrapper{internal: self.internal.#id().clone()}
                        }
                    }
                }
                "Circuit" => {
                    let msg = format!("Get value of struct field {}", id);
                    quote! {
                        #[doc = #msg]
                        pub fn #id(&self) -> CircuitWrapper{
                            CircuitWrapper{internal: self.internal.#id().clone()}
                        }
                    }
                }
                "Option<Circuit>" => {
                    let msg = format!("Get value of struct field {}", id);
                    quote! {
                            #[doc = #msg]
                            pub fn #id(&self) -> Option<CircuitWrapper>{
                                match self.internal.#id().as_ref(){
                                    None => None,
                                    Some(x) => Some(CircuitWrapper{internal: x.clone()})
                            }
                        }
                    }
                }
                _ => {
                    let msg = format!("Get value of struct field {}", id);
                    quote! {
                        #[doc = #msg]
                        pub fn #id(&self) -> #ty{
                            self.internal.#id().clone()
                        }
                    }
                }
            },
            _ => {
                let msg = format!("Get value of struct field {}", id);
                quote! {
                    #[doc=#msg]
                    pub fn #id(&self) -> #ty{
                        self.internal.#id().clone()
                    }
                }
            }
        });

    let new_msg = format!("Creates new instance of Operations {}", ident);
    quote! {

        #(#getter_fields)*

        #[new]
        #[doc = #new_msg]
        fn new(#(#input_arguments),*) -> PyResult<Self>{
            #(#conversion_quotes)*
            Ok(Self{internal: #ident::new(#(#arguments),*)})
        }

        /// Returns true if operation contains symbolic parameters
        ///
        /// Returns:
        ///     bool
        fn is_parametrized(&self) -> bool {
                self.internal.is_parametrized()
        }

        /// Returns tags identifying the Operation
        ///
        /// Returns:
        ///     list[str]: The tags identifying the operation
        fn tags(&self) -> Vec<String>{
            self.internal.tags().iter().map(|s| s.to_string()).collect()
        }

        /// Returns hqslang name of Operation
        ///
        /// Returns:
        ///     str: The name
        fn hqslang(&self) -> &'static str{
            self.internal.hqslang()
        }

        /// Substitutes internal symbolic parameters with float values
        ///
        /// Only available when all symbolic expressions can be evaluated to float with the
        /// provided parameters.
        ///
        /// Args:
        ///     substitution_parameters (dict[str, float]): The substituted free parameters
        ///
        /// Returns:
        ///     Operation: The operation with the parameters substituted
        ///
        /// Raises:
        ///     RuntimeError: Parameter Substitution failed
        fn substitute_parameters(&self, substitution_parameters: std::collections::HashMap<&str, f64>) -> PyResult<Self> {
            let mut calculator = qoqo_calculator::Calculator::new();
            for (key, val) in substitution_parameters.iter(){
                calculator.set_variable(key, *val);
            }
            Ok(Self{internal: self.internal.substitute_parameters(&calculator).map_err(|x| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Parameter Substitution failed: {:?}", x))
            })?})
        }

        /// Remap qubits
        ///
        /// Args:
        ///     mapping (dict[int, int]): The mapping
        ///
        /// Returns:
        ///     Operation: The operation with the remapped qubits
        ///
        /// Raises:
        ///     RuntimeError: Qubit remapping failed
        fn remap_qubits(&self, mapping: HashMap<usize, usize>) -> PyResult<Self>{
            let new_internal = self.internal.remap_qubits(&mapping).map_err(|x|
                PyRuntimeError::new_err(format!("Qubit remapping failed: {:?}",x))
            )?;
            Ok(Self{internal: new_internal})
        }

        /// List all involved Qubits
        ///
        /// Returns:
        ///     Union[set[int], str]: The involved qubits as a set or 'ALL' if all qubits are involved
        fn involved_qubits(&self) -> PyObject {
            Python::with_gil(|py| -> PyObject {
                let involved = self.internal.involved_qubits();
                match involved {
                    InvolvedQubits::All => {
                        let pyref: &PySet = PySet::new(py, &["All"]).unwrap();
                        let pyobject: PyObject = pyref.to_object(py);
                        pyobject
                    },
                    InvolvedQubits::None => {
                        let pyref: &PySet = PySet::empty(py).unwrap();
                        let pyobject: PyObject = pyref.to_object(py);
                        pyobject
                    },
                    InvolvedQubits::Set(x) => {
                        let mut vector: Vec<usize> = Vec::new();
                        for qubit in x {
                            vector.push(qubit)
                        }
                        let pyref: &PySet = PySet::new(py, &vector[..]).unwrap();
                        let pyobject: PyObject = pyref.to_object(py);
                        pyobject
                    },
                }
            })
        }

        /// Copies Operation
        ///
        /// For qoqo operations copy is always a deep copy
        fn __copy__(&self) -> Self {
            self.clone()
        }

        /// Creates deep copy of Operation
        fn __deepcopy__(&self, _memodict: Py<PyAny>) -> Self {
            self.clone()
        }
    }
}
