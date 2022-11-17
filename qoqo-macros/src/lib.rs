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

//! qoqo-macros
//!
//! Attribute proc-macros for the traits of qoqo [qoqo].

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    parse2, parse_macro_input, DataStruct, DeriveInput, Fields, GenericArgument, Ident, ItemImpl,
    ItemStruct, PathArguments, Token, Type, TypePath,
};
mod operate;
// mod operate_unitary;

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

// Struct for parsed derive macro arguments. Used to identify structs belonging to enums
#[derive(Debug)]
struct AttributeMacroArguments(HashSet<String>);

impl AttributeMacroArguments {
    pub fn contains(&self, st: &str) -> bool {
        self.0.contains(st)
    }
    pub fn _ids(&self) -> Vec<Ident> {
        self.0
            .clone()
            .into_iter()
            .map(|s| format_ident!("Wrap{}", s))
            .collect()
    }
}

impl Parse for AttributeMacroArguments {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse arguments as comma separated list of idents
        let arguments = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self(
            arguments.into_iter().map(|id| id.to_string()).collect(),
        ))
    }
}

/// Attribute macro for constructing the pyo3 wrappers for operation structs
#[proc_macro_attribute]
pub fn wrap(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attribute_arguments = parse_macro_input!(metadata as AttributeMacroArguments);
    let input2: TokenStream = input.clone().into();
    let parsed_input = parse_macro_input!(input as ItemStruct);
    let ident = parsed_input.ident;
    let struct_attributes = parsed_input.attrs;
    let str_ident = ident.to_string();
    let wrapper_ident = format_ident!("{}Wrapper", ident.to_string());
    let operate_quote = if attribute_arguments.contains("Operate") {
        derive_wrap_operate(input2)
    } else {
        TokenStream::new()
    };
    let rotate_quote = if attribute_arguments.contains("Rotate") {
        quote! {
            /// Returns angle of rotation
            pub fn theta(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{internal: self.internal.theta().clone()}
            }
            /// Returns Rotated gate raised to power
            ///
            /// Args:
            ///     `power`(CalculatorFloat): exponent of the power operation.
            ///
            /// Returns:
            ///     Self: gate raised to the power of `power`
            ///
            pub fn powercf(&self, power: CalculatorFloatWrapper) -> Self{
                Self{internal: self.internal.powercf(power.internal)}
            }
            #[cfg(feature = "overrotate")]
            /// Returns clone of the gate with one parameter statistically overrotated.
            fn overrotate(&self, amplitude: &f64, variance: &f64) -> Self {
                Self{internal: self.internal.overrotate(amplitude, variance)}

            }
        }
    } else {
        TokenStream::new()
    };
    let operate_pragma_quote = if attribute_arguments.contains("OperatePragma") {
        quote! {}
    } else {
        TokenStream::new()
    };
    let operate_pragma_noise_quote = if attribute_arguments.contains("OperatePragmaNoise") {
        quote! {
            /// Return the superoperator defining the evolution of the density matrix under the noise gate
            ///
            /// Returns:
            ///     np.ndarray
            ///
            pub fn superoperator(&self) -> PyResult<Py<PyArray2<f64>>>{
                Python::with_gil(|py| -> PyResult<Py<PyArray2<f64>>> {
                    Ok(self.internal.superoperator().unwrap().to_pyarray(py).to_owned())
                })
            }
            /// Return the power of the noise gate
            ///
            /// Args:
            ///     `power` (CalculatorFloat): exponent in the power operation of the noise gate
            ///
            /// Returns:
            ///     Self
            ///
            pub fn powercf(&self, power: CalculatorFloatWrapper) -> Self{
                Self{internal: self.internal.powercf(power.internal)}
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_pragma_noise_proba_quote =
        if attribute_arguments.contains("OperatePragmaNoiseProba") {
            quote! {
                /// Returns the probability associated with the noise operation
                ///
                /// Returns:
                ///     CalculatorFloat
                pub fn probability(&self) -> CalculatorFloatWrapper{
                    CalculatorFloatWrapper{internal: self.internal.probability().clone()}
                }
            }
        } else {
            TokenStream::new()
        };
    let operate_single_qubit_quote = if attribute_arguments.contains("OperateSingleQubit") {
        quote! {
            /// Return the qubit the operation acts on
            ///
            /// Returns:
            ///     int
            pub fn qubit(&self) -> usize{
                self.internal.qubit().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_single_qubit_gate_quote = if attribute_arguments.contains("OperateSingleQubitGate")
    {
        quote! {
            /// Return the global phase :math:`g` of a unitary gate acting on one qubit
            ///
            /// Here global_phase is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn global_phase(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{internal: self.internal.global_phase().clone()}
            }
            /// Return the property alpha_r :math:`\alpha_r` of a unitary gate acting on one qubit
            ///
            /// Here alpha_r is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn alpha_r(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{internal: self.internal.alpha_r().clone()}
            }
            /// Return the property alpha_i :math:`\alpha_i` of a unitary gate acting on one qubit
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn alpha_i(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{internal: self.internal.alpha_i().clone()}
            }
            /// Return the property beta_r :math:`\beta_r` of a unitary gate acting on one qubit
            ///
            /// Here beta_r is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn beta_r(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{internal: self.internal.beta_r().clone()}
            }
            /// Returns the property beta_i :math:`\beta_i` of a unitary gate acting on one qubit
            ///
            /// Here beta_i is defined by
            ///
            /// .. math::
            ///     U =e^{i \cdot g}\begin{pmatrix}
            ///     \alpha_r+i \alpha_i & -\beta_r+i \beta_i \\\\
            ///     \beta_r+i \beta_i & \alpha_r-i\alpha_i
            ///     \end{pmatrix}
            ///
            ///
            /// Returns:
            ///     CalculatorFloat
            pub fn beta_i(&self) -> CalculatorFloatWrapper{
                CalculatorFloatWrapper{internal: self.internal.beta_i().clone()}
            }

            /// Multiplies two compatible operations implementing OperateSingleQubitGate.
            ///
            /// Does not consume the two operations being multiplied.
            /// Only Operations
            ///
            /// Args:
            ///     `other` - An Operation implementing [OperateSingleQubitGate].
            ///
            /// Returns:
            ///     PyResult: Result of the multiplication, i.e. the multiplied single qubit gate.
            ///
            /// Example:
            /// ```
            /// from qoqo.operations import RotateZ, RotateX
            ///
            /// gate1 =  RotateZ(qubit=0, theta=1)
            /// gate2 = RotateX(qubit=0, theta=1)
            /// multiplied = gate1.mul(gate2)
            /// print("Multiplied gate: ", multiplied)
            /// ```
            ///
            pub fn mul(&self, other: Py<PyAny>) -> PyResult<SingleQubitGateWrapper> {
                Python::with_gil(|py| -> PyResult<SingleQubitGateWrapper> {
                    let other_ref = other.as_ref(py);
                    let other: Operation = crate::operations::convert_pyany_to_operation(other_ref).map_err(|x| {
                        pyo3::exceptions::PyTypeError::new_err(format!("Right hand side cannot be converted to Operation {:?}",x))
                    })?;
                    let other_converted: SingleQubitGateOperation = other.clone().try_into().map_err(|x| {
                        pyo3::exceptions::PyRuntimeError::new_err(format!("Conversion to SingleQubitGateOperation failed {:?}",x))
                    })?;
                    let multiplied = self.internal.mul(&other_converted).map_err(|x| {
                        pyo3::exceptions::PyRuntimeError::new_err(format!("Multiplication failed {:?}",x))
                    })?;
                    Ok(SingleQubitGateWrapper{ internal: multiplied})
                })
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_two_qubit_quote = if attribute_arguments.contains("OperateTwoQubit") {
        quote! {
            /// Returns contol qubit of the two-qubit operation
            pub fn control(&self) -> usize{
                self.internal.control().clone()
            }
            /// Returns target qubit of the two-qubit operation
            pub fn target(&self) -> usize{
                self.internal.target().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    // let operate_two_qubit_gate_quote = if attribute_arguments.contains("OperateTwoQubitGate") {
    //     quote! {
    //         /// Returns kak decomposition of the two-qubit-gate operation
    //         pub fn kak_decomposition(&self) -> KakDecompositionWrapper {
    //             KakDecompositionWrapper{internal: self.internal.kak_decomposition().clone()}
    //         }
    //     }
    // } else {
    //     TokenStream::new()
    // };
    let operate_gate_quote = if attribute_arguments.contains("OperateGate") {
        quote! {
            /// Return unitary matrix of gate.
            ///
            /// Returns:
            ///     np.ndarray
            ///
            /// Raises:
            ///     ValueError: Error symbolic operation cannot return float unitary matrix
            pub fn unitary_matrix(&self) -> PyResult<Py<PyArray2<Complex64>>>{
                Python::with_gil(|py| -> PyResult<Py<PyArray2<Complex64>>> {
                    Ok(self.internal.unitary_matrix().map_err(|x| PyValueError::new_err(format!("Error symbolic operation cannot return float unitary matrix {:?}",x)))?.to_pyarray(py).to_owned())
                })
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_multi_qubit_quote = if attribute_arguments.contains("OperateMultiQubit") {
        quote! {
            /// Return list of qubits of the multi qubit operation in order of descending significance
            ///
            /// Returns:
            ///     list[int]
            pub fn qubits(&self) -> Vec<usize>{
                self.internal.qubits().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_multi_qubit_gate_quote = if attribute_arguments.contains("OperateMultiQubitGate") {
        quote! {
            /// Return circuit implementing MultiQubitGateOperation
            ///
            /// Returns:
            ///     Circuit
            pub fn circuit(&self) -> CircuitWrapper{
                CircuitWrapper { internal: self.internal.circuit().clone() }
            }
        }
    } else {
        TokenStream::new()
    };
    let define_quote = if attribute_arguments.contains("Define") {
        quote! {
        /// Return name of definition operation.
        ///
        /// Returns:
        ///     str
        pub fn name(&self) -> String {
                self.internal.name().clone()
            }
        }
    } else {
        TokenStream::new()
    };
    let operate_constant_gate_quote = if attribute_arguments.contains("OperateConstantGate") {
        quote! {
        /// Return inverse of GateOperation:
        ///
        /// Returns:
        ///     GateOperation
        pub fn inverse(&self) -> GateOperationWrapper {
                GateOperationWrapper { internal: self.internal.inverse().clone() }
            }
        }
    } else {
        TokenStream::new()
    };
    let msg = format!("Internal storage of {} object", ident);
    let q = quote! {
        #[automatically_derived]
        #[pyclass(name=#str_ident)]
        #(#struct_attributes)*
        #[derive(Debug, Clone, PartialEq)]
        pub struct #wrapper_ident{
            #[doc = #msg]
            pub internal: #ident
        }
        #[automatically_derived]
        #[pymethods]
        impl #wrapper_ident{
            #operate_quote
            #operate_single_qubit_quote
            #operate_single_qubit_gate_quote
            #operate_two_qubit_quote
            // #operate_two_qubit_gate_quote
            #operate_multi_qubit_quote
            #operate_multi_qubit_gate_quote
            #operate_gate_quote
            #rotate_quote
            #operate_pragma_quote
            #operate_pragma_noise_quote
            #operate_pragma_noise_proba_quote
            #define_quote
            #operate_constant_gate_quote
            fn __format__(&self, _format_spec: &str) -> PyResult<String> {
                Ok(format!("{:?}", self.internal))
            }

            fn __repr__(&self) -> PyResult<String> {
                Ok(format!("{:?}", self.internal))
            }


            /// Returns the __richcmp__ magic method to perform rich comparison
            /// operations on Operation.
            ///
            /// Args:
            ///
            /// * `&self` - the OperationWrapper object
            /// * `other` - the object to compare self to
            /// * `op` - equal or not equal
            ///
            /// Returns:
            ///
            /// `PyResult<bool>` - whether the two operations compared evaluated to True or False
            ///
            fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
                Python::with_gil(|py| -> PyResult<bool> {
                    let other_ref = other.as_ref(py);
                    let other: Operation = crate::operations::convert_pyany_to_operation(other_ref).map_err(|x| {
                        pyo3::exceptions::PyTypeError::new_err(format!("Right hand side cannot be converted to Operation {:?}",x))
                    })?;
                    match op {
                        pyo3::class::basic::CompareOp::Eq => Ok(Operation::from(self.internal.clone()) == other),
                        pyo3::class::basic::CompareOp::Ne => Ok(Operation::from(self.internal.clone()) != other),
                        _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                            "Other comparison not implemented.",
                        )),
                    }
                })
            }

        }
    };
    q.into()
}

fn derive_wrap_operate(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse2(input).unwrap();
    operate::dispatch_struct(parsed_input)
}

/// Macro for injecting code to convert PyAny to Operation
#[proc_macro]
pub fn insert_pyany_to_operation(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {})
}

/// Macro for injecting code to convert PyAny to Operation
#[proc_macro]
pub fn insert_operation_to_pyobject(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {})
}

/// Extrats the identifier of fields of a named struct
/// together with the optional cast of the type to string form (where the type is a simple path) and the type as a syn object
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

// A macro to generate impl Device Wrapper for qoqo devices
#[proc_macro_attribute]
pub fn devicewrapper(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as ItemImpl);
    let ident = parsed_input.self_ty;
    let items = parsed_input.items;
    let q = quote! {
        #[pymethods]
        impl #ident {
            #(#items)*

            /// Return number of qubits in device.
            ///
            /// Returns:
            ///     int: The number of qubits.
            pub fn number_qubits(&self) -> usize {
                self.internal.number_qubits()
            }

            /// Return the list of pairs of qubits linked by a native two-qubit-gate in the device.
            ///
            /// A pair of qubits is considered linked by a native two-qubit-gate if the device
            /// can implement a two-qubit-gate between the two qubits without decomposing it
            /// into a sequence of gates that involves a third qubit of the device.
            /// The two-qubit-gate also has to form a universal set together with the available
            /// single qubit gates.
            ///
            /// The returned vectors is a simple, graph-library independent, representation of
            /// the undirected connectivity graph of the device.
            /// It can be used to construct the connectivity graph in a graph library of the user's
            /// choice from a list of edges and can be used for applications like routing in quantum algorithms.
            ///
            /// Returns:
            ///     Sequence[(int, int)]: List of two qubit edges in the undirected connectivity graph
            ///
            pub fn two_qubit_edges(&self) -> Vec<(usize, usize)> {
                self.internal.two_qubit_edges()
            }

            /// Returns the gate time of a single qubit operation if the single qubit operation is available on device.
            ///
            /// Args:
            ///     hqslang[str]: The hqslang name of a single qubit gate.
            ///     qubit[int]: The qubit the gate acts on
            ///
            /// Returns:
            ///     Option[float]: None if gate is not available
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, qubit)")]
            pub fn single_qubit_gate_time(&self, hqslang: &str, qubit: usize) -> Option<f64> {
                self.internal.single_qubit_gate_time(hqslang, &qubit)
            }

            /// Returns the gate time of a two qubit operation if the two qubit operation is available on device.
            ///
            /// Args:
            ///     hqslang[str]: The hqslang name of a single qubit gate.
            ///     control[int]: The control qubit the gate acts on.
            ///     target[int]: The target qubit the gate acts on.
            ///
            /// Returns:
            ///     Option[float]: None if gate is not available
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            ///
            #[pyo3(text_signature = "(gate, control, target)")]
            pub fn two_qubit_gate_time(&self, hqslang: &str, control: usize, target: usize) -> Option<f64> {
                self.internal
                    .two_qubit_gate_time(hqslang, &control, &target)
            }

            /// Returns the gate time of a multi qubit operation if the multi qubit operation is available on device.
            ///
            /// Args:
            ///     hqslang[str]: The hqslang name of a multi qubit gate.
            ///     qubits[List[int]]: The qubits the gate acts on.
            ///
            /// Returns:
            ///     Option[float]: None if gate is not available
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, qubits)")]
            pub fn multi_qubit_gate_time(&self, hqslang: &str, qubits: Vec<usize>) -> Option<f64> {
                self.internal.multi_qubit_gate_time(hqslang, &qubits)
            }

            /// Set the gate time of a single qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     qubit (int): The qubit for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(gate, qubit, gate_time)")]
            pub fn set_single_qubit_gate_time(&mut self, gate: &str, qubit: usize, gate_time: f64) -> PyResult<()> {
                self.internal.set_single_qubit_gate_time(gate, qubit, gate_time).map_err(|err|
                PyValueError::new_err(format!("{:?}", err)))
            }

            /// Set the gate time of a single qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     control (int): The control qubit for which the gate time is set
            ///     target (int): The control qubit for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, control, targe, gate_time)")]
            pub fn set_two_qubit_gate_time(&mut self, gate: &str, control: usize, target: usize, gate_time: f64) -> PyResult<()> {
                self.internal.set_two_qubit_gate_time(gate, control, target, gate_time).map_err(|err|
                    PyValueError::new_err(format!("{:?}", err)))
            }


            /// Set the gate time of a single qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     qubits (int): The qubit for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, rates)")]
            pub fn set_qubit_decoherence_rates(&mut self, qubit: usize, rates: PyReadonlyArray2<f64>) -> PyResult<()> {
                let rates_matrix = rates.as_array().to_owned();
                self.internal
                    .set_qubit_decoherence_rates(qubit, rates_matrix)
                    .map_err(|err| {
                        PyValueError::new_err(format!("Could not set rates: {}", err))
                    })
            }

            /// Set the gate time of a single qubit gate.
            ///
            /// Args:
            ///     gate (str): hqslang name of the single-qubit-gate.
            ///     qubits (List[int]): The qubits for which the gate time is set
            ///     gate_time (float): The gate time for the given gate.
            ///
            /// Raises:
            ///     PyValueError: Qubits not in device
            #[pyo3(text_signature = "(gate, qubits, gate_time)")]
            pub fn set_multi_qubit_gate_time(&self, gate: &str, qubits: Vec<usize>, gate_time: f64) -> PyResult<()> {
                self.internal.clone().set_multi_qubit_gate_time(gate, qubits, gate_time).map_err(|err|
                    PyValueError::new_err(format!("{:?}", err)))
            }

            /// Return the matrix of the decoherence rates of the Lindblad equation.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the rate matrix M is returned
            ///
            /// Returns:
            ///     numpy.array: 3 by 3 numpy array of decoherence rates
            ///
            #[pyo3(text_signature = "(qubit)")]
            fn qubit_decoherence_rates(&self, qubit: usize) -> Py<PyArray2<f64>> {
                Python::with_gil(|py| -> Py<PyArray2<f64>> {
                    match self.internal.qubit_decoherence_rates(&qubit) {
                        Some(matrix) => matrix.to_pyarray(py).to_owned(),
                        None => {
                            let matrix = Array2::<f64>::zeros((3, 3));
                            matrix.to_pyarray(py).to_owned()
                        }
                    }
                })
            }

            /// Adds single qubit damping to noise rates.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the decoherence is added
            ///     damping (float): The damping rates.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, damping)")]
            pub fn add_damping(&mut self, qubit: usize, damping: f64) -> PyResult<()> {
                self.internal.add_damping(qubit,damping).map_err(|err| PyValueError::new_err(format!("Cannot add decoherence: {}",err)))
            }

             /// Adds single qubit dephasing to noise rates.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the decoherence is added
            ///     dephasing (float): The dephasing rates.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, dephasing)")]
            pub fn add_dephasing(&mut self, qubit: usize, dephasing: f64) -> PyResult<()> {
                self.internal.add_dephasing(qubit,dephasing).map_err(|err| PyValueError::new_err(format!("Cannot add decoherence: {}",err)))

            }

            /// Adds single qubit depolarising to noise rates.
            ///
            /// Args:
            ///     qubit (int): The qubit for which the decoherence is added
            ///     depolarising (float): The depolarising rates.
            ///
            /// Raises:
            ///     PyValueError: Qubit is not in device
            #[pyo3(text_signature = "(qubit, depolarising)")]
            pub fn add_depolarising(&mut self, qubit: usize, depolarising: f64) -> PyResult<()> {
                self.internal.add_depolarising(qubit,depolarising).map_err(|err| PyValueError::new_err(format!("Cannot add decoherence: {}",err)))
            }


            /// Turns Device into GenericDevice
            ///
            /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
            /// (for example when the interface needs to be serialized)
            ///
            /// Returns:
            ///     GenericDevice: The device in generic representation
            ///
            /// Note:
            ///     GenericDevice uses nested HashMaps to represent the most general device connectivity.
            ///     The memory usage will be inefficient for devices with large qubit numbers.
            fn generic_device(&self) -> GenericDeviceWrapper {
                GenericDeviceWrapper{ internal: self.internal.to_generic_device()}
            }

            /// Turns Device into GenericDevice
            ///
            /// Can be used as a generic interface for devices when a boxed dyn trait object cannot be used
            /// (for example when the interface needs to be serialized)
            ///
            /// Returns:
            ///     GenericDevice: The device in generic representation
            ///
            /// Note:
            ///     GenericDevice uses nested HashMaps to represent the most general device connectivity.
            ///     The memory usage will be inefficient for devices with large qubit numbers.
            fn to_generic_device(&self) -> GenericDeviceWrapper {
                GenericDeviceWrapper{ internal: self.internal.to_generic_device()}
            }

            /// Returns a copy of the device (copy here produces a deepcopy).
            ///
            /// Returns:
            ///     A deep copy of self.
            ///
            pub fn __copy__(&self) -> Self {
                self.clone()
            }

            /// Creates deep copy of Device.
            ///
            /// Returns:
            ///     A deep copy of self.
            ///
            pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> Self {
                self.clone()
            }

            /// Return the bincode representation of the Device using the bincode crate.
            ///
            /// Returns:
            ///     ByteArray: The serialized Device (in bincode form).
            ///
            /// Raises:
            ///     ValueError: Cannot serialize Device to bytes.
            ///
            pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
                let serialized = serialize(&self.internal)
                    .map_err(|_| PyValueError::new_err("Cannot serialize Device to bytes"))?;
                let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
                    PyByteArray::new(py, &serialized[..]).into()
                });
                Ok(b)
            }

            /// Return the json representation of the Device.
            ///
            /// Returns:
            ///     str: The serialized form of Device.
            ///
            /// Raises:
            ///     ValueError: Cannot serialize Device to json.
            ///
            pub fn to_json(&self) -> PyResult<String> {
                let serialized = serde_json::to_string(&self.internal)
                    .map_err(|_| PyValueError::new_err("Cannot serialize Device to json"))?;
                Ok(serialized)
            }

            /// Convert the bincode representation of the qoqo device to a device using the bincode crate.
            ///
            /// Args:
            ///     input (ByteArray): The serialized Device (in bincode form).
            ///
            /// Returns:
            ///     The deserialized Device.
            ///
            /// Raises:
            ///     TypeError: Input cannot be converted to byte array.
            ///     ValueError: Input cannot be deserialized to selected Device.
            #[staticmethod]
            #[pyo3(text_signature = "(input)")]
            pub fn from_bincode(input: &PyAny) -> PyResult<#ident> {
                let bytes = input
                    .extract::<Vec<u8>>()
                    .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

                Ok(#ident {
                    internal: deserialize(&bytes[..]).map_err(|_| {
                        PyValueError::new_err("Input cannot be deserialized to selected Device.")
                    })?,
                })
            }

            /// Convert the json representation of a device to a qoqo device.
            ///
            /// Args:
            ///     input (str): The serialized device in json form.
            ///
            /// Returns:
            ///     The deserialized device.
            ///
            /// Raises:
            ///     ValueError: Input cannot be deserialized to selected Device.
            #[staticmethod]
            #[pyo3(text_signature = "(input)")]
            pub fn from_json(input: &str) -> PyResult<#ident> {
                Ok(#ident {
                    internal: serde_json::from_str(input).map_err(|_| {
                        PyValueError::new_err("Input cannot be deserialized to selected Device.")
                    })?,
                })
            }

            fn __repr__(&self) -> String{
                format!("{:?}", self.internal)
            }


            /// Return the __richcmp__ magic method to perform rich comparison operations on mixed system.
            ///
            /// Args:
            ///     other: The object to compare self to.
            ///     op: Whether they should be equal or not.
            ///
            /// Returns:
            ///     bool
            ///
            /// Raises:
            ///     NotImplementedError: Other comparison not implemented.
            ///
            fn __richcmp__(&self, other: Py<PyAny>, op: pyo3::class::basic::CompareOp) -> PyResult<bool> {
                let other = #ident::from_pyany(other);
                match op {
                    pyo3::class::basic::CompareOp::Eq => match other {
                        Ok(osystem) => Ok(self.internal == osystem),
                        _ => Ok(false),
                    },
                    pyo3::class::basic::CompareOp::Ne => match other {
                        Ok(osystem) => Ok(self.internal != osystem),
                        _ => Ok(true),
                    },
                    _ => Err(pyo3::exceptions::PyNotImplementedError::new_err(
                        "Other comparison not implemented",
                    )),
                }
            }
        }
    };
    q.into()
}
